#!/usr/bin/env python3
"""🌙️ Fleet scope reconnaissance — enumerate crates, measure per-crate error ownership.

GOAL
----
For fleet repair fan-out (33 plugins + 26 extension crates), determine the actual repair
cost per crate by distinguishing:
  * Own errors: primary span in this crate's files (expensive, needs design)
  * Inherited errors: primary span in 🧰️framework/* (cheap, shared fix upstream)

DELIVERABLES
1. Crate enumeration from workspace Cargo.toml
2. Per-crate error classification (own vs inherited)
3. Error taxonomy (code + await suggestion fraction)
4. Proposed partition into ≤8 batches by repair cost
"""

from __future__ import annotations

import json
import os
import re
import subprocess
import sys
from collections import defaultdict
from dataclasses import dataclass, asdict
from pathlib import Path

REPO = Path("/Users/ueli/Documents/semio")
SCRATCH = Path("/private/tmp/claude-501/-Users-ueli-Documents-semio/"
               "e6a44461-bab7-421f-8a53-65123a5e9482/scratchpad")
TARGET_DIR = SCRATCH / "target-luna"
TICKET = (REPO / ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17"
          "/MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME")

@dataclass
class PerCrateAnalysis:
    """Analysis results for one fleet crate."""
    crate_name: str
    path: str
    total_errors: int
    own_errors: int
    inherited_errors: int
    build_status: str  # "ok", "failed", "missing_target"
    error_codes: dict[str, int]  # code -> count
    await_suggestions_count: int  # errors with await suggested_replacement
    await_fraction: float  # await_suggestions_count / own_errors if > 0

def _extract_toml_name(path: Path) -> str | None:
    """🔍 Extract package name from Cargo.toml using regex."""
    try:
        with open(path, "r", encoding="utf-8") as fh:
            content = fh.read()
        match = re.search(r'name\s*=\s*["\']([^"\']+)["\']', content)
        return match.group(1) if match else None
    except Exception:
        return None

def enumerate_fleet_crates() -> tuple[list[tuple[str, str]], list[tuple[str, str]]]:
    """🔎 Extract fleet crates from workspace Cargo.toml.

    Returns (plugins, extensions) where each is [(crate_name, workspace_path)]
    """
    with open(REPO / "Cargo.toml", "r", encoding="utf-8") as fh:
        workspace_content = fh.read()

    # Extract members array
    members_match = re.search(r'\[workspace\].*?members\s*=\s*\[(.*?)\]',
                               workspace_content, re.DOTALL)
    if not members_match:
        print("[ERROR] Could not find members array in Cargo.toml", file=sys.stderr)
        return [], []

    members_str = members_match.group(1)
    member_paths = []
    for m in re.finditer(r'"([^"]+)"', members_str):
        member_paths.append(m.group(1))

    plugins = []
    extensions = []

    for member_path in member_paths:
        cargo_path = REPO / member_path / "Cargo.toml"
        if not cargo_path.exists():
            continue

        crate_name = _extract_toml_name(cargo_path)
        if not crate_name:
            continue

        # Classify: fleet plugin (semio-s-plugin-*) vs extension (flows, process, etc)
        if crate_name.startswith("semio-s-plugin-"):
            plugins.append((crate_name, member_path))
        elif ("extension" in crate_name.lower() or
              "extensio" in member_path.lower()):
            extensions.append((crate_name, member_path))

    return sorted(plugins), sorted(extensions)

def run_check(crate: str, target_dir: Path) -> tuple[list[dict], str]:
    """🩺 Run cargo check -p <crate> --lib and return (diagnostics, status).

    Returns (messages, status) where status is "ok", "failed", or "missing_target".
    """
    cmd = [
        "cargo", "check", "-p", crate, "--lib",
        "--message-format=json",
    ]
    env = dict(os.environ, CARGO_TARGET_DIR=str(target_dir))
    try:
        proc = subprocess.run(
            cmd, cwd=str(REPO), env=env, capture_output=True, text=True,
            timeout=600
        )
    except subprocess.TimeoutExpired:
        return [], "timeout"

    if proc.returncode not in (0, 101):  # 101 = compilation error (expected)
        stderr = proc.stderr[:500]
        if "no library targets found" in stderr or "could not compile" in stderr:
            return [], "missing_target"
        return [], "failed"

    diags = []
    for line in proc.stdout.splitlines():
        line = line.strip()
        if not line.startswith("{"):
            continue
        try:
            msg = json.loads(line)
        except json.JSONDecodeError:
            continue
        if msg.get("reason") == "compiler-message" and msg.get("message"):
            diags.append(msg["message"])

    return diags, "ok"

def is_own_error(diag: dict) -> bool:
    """🎯 True if the primary span is in this crate's own files, not 🧰️framework/."""
    for span in diag.get("spans", []):
        if not span.get("is_primary"):
            continue
        path = span.get("file_name", "")
        return "🧰️framework/" not in path
    return False

def count_await_suggestions(diag: dict) -> int:
    """🔍 Count how many suggested_replacement strings contain an await token."""
    count = 0
    def walk(node):
        nonlocal count
        for child in node.get("children", []):
            for span in child.get("spans", []):
                repl = span.get("suggested_replacement", "").lower()
                if "await" in repl:
                    count += 1
            walk(child)
    walk(diag)
    return count

def analyze_crate(crate_name: str, target_dir: Path) -> PerCrateAnalysis:
    """📊 Run cargo check and classify errors for one crate."""
    diags, status = run_check(crate_name, target_dir)

    if status != "ok":
        return PerCrateAnalysis(
            crate_name=crate_name, path="", total_errors=0, own_errors=0,
            inherited_errors=0, build_status=status, error_codes={},
            await_suggestions_count=0, await_fraction=0.0
        )

    errors = [d for d in diags if d.get("level") == "error"]
    own = sum(1 for e in errors if is_own_error(e))
    inherited = len(errors) - own

    # Error code breakdown (own errors only)
    codes = defaultdict(int)
    await_count = 0
    for error in errors:
        if not is_own_error(error):
            continue
        code = (error.get("code") or {}).get("code") or "(no-code)"
        codes[code] += 1
        await_count += count_await_suggestions(error)

    await_frac = await_count / own if own > 0 else 0.0

    return PerCrateAnalysis(
        crate_name=crate_name, path="", total_errors=len(errors),
        own_errors=own, inherited_errors=inherited, build_status=status,
        error_codes=dict(codes), await_suggestions_count=await_count,
        await_fraction=await_frac
    )

def partition_batches(analyses: list[PerCrateAnalysis]) -> list[list[PerCrateAnalysis]]:
    """🔀 Partition crates into ≤8 batches of roughly equal repair cost.

    Cost heuristic:
      * Build failure: 0 (blocked)
      * ~100% await: own_errors * 0.1 (mechanical)
      * ~50% await:  own_errors * 0.5 (mixed)
      * ~0% await:   own_errors * 1.0 (expensive design work)
    """
    def cost(a: PerCrateAnalysis) -> float:
        if a.build_status != "ok" or a.own_errors == 0:
            return 0.0
        await_frac = a.await_fraction
        # Interpolate: 0% await -> 1.0, 100% await -> 0.1
        multiplier = 1.0 - (await_frac * 0.9)
        return a.own_errors * multiplier

    # Sort by cost descending so we pack big items first
    sorted_analyses = sorted(analyses, key=cost, reverse=True)

    # Greedy bin-packing into up to 8 bins
    batches = [[] for _ in range(8)]
    batch_costs = [0.0] * 8

    for analysis in sorted_analyses:
        c = cost(analysis)
        # Find bin with lowest current cost
        min_idx = batch_costs.index(min(batch_costs))
        batches[min_idx].append(analysis)
        batch_costs[min_idx] += c

    # Remove empty batches
    return [b for b in batches if b]

def main() -> int:
    os.makedirs(TARGET_DIR, exist_ok=True)

    print("🌙️ Luna Fleet Scope Reconnaissance")
    print("=" * 70)

    print("\n[1] Enumerating fleet crates...")
    plugins, extensions = enumerate_fleet_crates()
    all_fleet = plugins + extensions

    print(f"  {len(plugins)} plugin crates (semio-s-plugin-*)")
    print(f"  {len(extensions)} extension crates")
    print(f"  {len(all_fleet)} total fleet crates")

    print("\n[2] Running cargo check per crate...")
    print("  (this may take 5-10 minutes; per-crate JSON will be written to ticket folder)")

    analyses = []
    for i, (crate_name, workspace_path) in enumerate(all_fleet, 1):
        print(f"  [{i:2d}/{len(all_fleet)}] {crate_name}...", end=" ", flush=True)
        analysis = analyze_crate(crate_name, TARGET_DIR)
        analyses.append(analysis)

        # Write per-crate JSON to ticket folder
        json_file = TICKET / f"luna-fleet-{crate_name}.json"
        with open(json_file, "w") as fh:
            json.dump(asdict(analysis), fh, indent=2)

        print(f"own={analysis.own_errors} inherited={analysis.inherited_errors} "
              f"status={analysis.build_status}")

    print("\n[3] Generating report...")

    # Summary statistics
    total_own = sum(a.own_errors for a in analyses)
    total_inherited = sum(a.inherited_errors for a in analyses)
    total_errors = sum(a.total_errors for a in analyses)
    crates_ok = sum(1 for a in analyses if a.build_status == "ok")
    crates_failed = sum(1 for a in analyses if a.build_status == "failed")
    crates_missing = sum(1 for a in analyses if a.build_status == "missing_target")

    # Error codes across all crates
    all_codes = defaultdict(int)
    for a in analyses:
        for code, count in a.error_codes.items():
            all_codes[code] += count

    # Await fraction across own errors
    total_await = sum(a.await_suggestions_count for a in analyses)
    await_frac_all = total_await / total_own if total_own > 0 else 0.0

    # Partition into batches
    batches = partition_batches(analyses)

    # Write markdown report
    report_path = TICKET / "📓️luna-fleet-scope-report.md"
    with open(report_path, "w", encoding="utf-8") as fh:
        fh.write("# Luna Fleet Scope Report\n\n")
        fh.write(f"**Date:** 2026-08-20  \n")
        fh.write(f"**Analysis:** per-crate error ownership classification  \n\n")

        fh.write("## Executive Summary\n\n")
        fh.write(f"- **Total fleet crates:** {len(all_fleet)} ({len(plugins)} plugins + {len(extensions)} extensions)\n")
        fh.write(f"- **Build status:** {crates_ok} OK, {crates_failed} failed, {crates_missing} missing target\n")
        fh.write(f"- **Total diagnostic errors:** {total_errors:,}\n")
        fh.write(f"- **Own errors (this crate's files):** {total_own:,}\n")
        fh.write(f"- **Inherited errors (🧰️framework/*):** {total_inherited:,}\n")
        fh.write(f"- **Fraction of own errors with await suggestion:** {await_frac_all:.1%}\n\n")

        fh.write("## Error Code Breakdown (Own Errors Only)\n\n")
        fh.write("| Code | Count | % of own |\n")
        fh.write("|------|-------|----------|\n")
        for code in sorted(all_codes, key=lambda c: all_codes[c], reverse=True)[:20]:
            count = all_codes[code]
            pct = 100.0 * count / total_own if total_own > 0 else 0
            fh.write(f"| {code} | {count:,} | {pct:.1f}% |\n")
        fh.write("\n")

        fh.write("## Per-Crate Analysis\n\n")
        fh.write("| Crate | Total | Own | Inherited | Status | Await% | Effort |\n")
        fh.write("|-------|-------|-----|-----------|--------|--------|--------|\n")

        for a in sorted(analyses, key=lambda x: x.own_errors, reverse=True):
            effort = "Blocked" if a.build_status != "ok" else (
                "Low" if a.await_fraction > 0.7 else (
                    "Med" if a.await_fraction > 0.3 else "High"
                )
            )
            frac_str = f"{a.await_fraction:.0%}" if a.own_errors > 0 else "-"
            fh.write(
                f"| {a.crate_name} | {a.total_errors} | {a.own_errors} | "
                f"{a.inherited_errors} | {a.build_status} | {frac_str} | {effort} |\n"
            )
        fh.write("\n")

        fh.write("## Proposed Batch Partition\n\n")
        fh.write("Crates grouped for parallel repair fan-out. Cost metric:\n")
        fh.write("- Blocked crates (failed build) contribute 0 cost\n")
        fh.write("- Mechanical (>70% await) cost = own_errors × 0.1\n")
        fh.write("- Mixed (30-70% await) cost = own_errors × 0.5\n")
        fh.write("- Design-heavy (<30% await) cost = own_errors × 1.0\n\n")

        for batch_idx, batch in enumerate(batches, 1):
            batch_cost = sum(
                max(0, (1.0 - (a.await_fraction * 0.9)) * a.own_errors)
                for a in batch if a.build_status == "ok"
            )
            total_batch_own = sum(a.own_errors for a in batch)
            fh.write(f"### Batch {batch_idx}\n\n")
            fh.write(f"**Cost estimate:** {batch_cost:.0f} (total own errors: {total_batch_own})  \n\n")
            fh.write(f"```\n")
            for a in sorted(batch, key=lambda x: x.own_errors, reverse=True):
                effort = "Low" if a.await_fraction > 0.7 else (
                    "Med" if a.await_fraction > 0.3 else "High"
                )
                frac_str = f"{a.await_fraction:.0%}" if a.own_errors > 0 else "-"
                fh.write(f"{a.crate_name:45} own={a.own_errors:5} await={frac_str:>5} {effort}\n")
            fh.write(f"```\n\n")

    print(f"✓ Report written to {report_path}")
    print(f"\n[Summary]")
    print(f"  Fleet: {len(all_fleet)} crates ({len(plugins)} + {len(extensions)})")
    print(f"  Own errors: {total_own:,} | Inherited: {total_inherited:,}")
    print(f"  Await-eligible: {await_frac_all:.1%}")
    print(f"  Batches: {len(batches)} (up to 8)")

    return 0

if __name__ == "__main__":
    sys.exit(main())
