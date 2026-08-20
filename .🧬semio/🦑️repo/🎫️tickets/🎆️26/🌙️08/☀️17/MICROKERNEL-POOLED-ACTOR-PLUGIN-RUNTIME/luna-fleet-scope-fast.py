#!/usr/bin/env python3
"""🌙️ Fleet scope reconnaissance — optimized with parallel cargo checks."""

from __future__ import annotations

import concurrent.futures
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
    crate_name: str
    total_errors: int
    own_errors: int
    inherited_errors: int
    build_status: str
    error_codes: dict[str, int]
    await_suggestions_count: int
    await_fraction: float

def enumerate_fleet_crates():
    """Extract fleet crates from workspace."""
    def _extract_toml_name(path):
        try:
            with open(path, "r", encoding="utf-8") as fh:
                content = fh.read()
            match = re.search(r'name\s*=\s*["\']([^"\']+)["\']', content)
            return match.group(1) if match else None
        except Exception:
            return None

    with open(REPO / "Cargo.toml", "r", encoding="utf-8") as fh:
        workspace_content = fh.read()

    members_match = re.search(r'\[workspace\].*?members\s*=\s*\[(.*?)\]',
                               workspace_content, re.DOTALL)
    members_str = members_match.group(1)
    member_paths = [m.group(1) for m in re.finditer(r'"([^"]+)"', members_str)]

    plugins = []
    extensions = []

    for member_path in member_paths:
        cargo_path = REPO / member_path / "Cargo.toml"
        if not cargo_path.exists():
            continue
        crate_name = _extract_toml_name(cargo_path)
        if not crate_name:
            continue
        if crate_name.startswith("semio-s-plugin-"):
            plugins.append(crate_name)
        elif "extension" in crate_name.lower() or "extensio" in member_path.lower():
            extensions.append(crate_name)

    return sorted(plugins), sorted(extensions)

def is_own_error(diag: dict) -> bool:
    """True if primary span is in this crate's files."""
    for span in diag.get("spans", []):
        if span.get("is_primary"):
            return "🧰️framework/" not in span.get("file_name", "")
    return False

def count_await_suggestions(diag: dict) -> int:
    """Count await suggestions in diagnostic tree."""
    count = 0
    def walk(node):
        nonlocal count
        for child in node.get("children", []):
            for span in child.get("spans", []):
                if "await" in (span.get("suggested_replacement", "") or "").lower():
                    count += 1
            walk(child)
    walk(diag)
    return count

def analyze_crate_safe(crate_name: str, target_dir: Path) -> PerCrateAnalysis:
    """Run cargo check with timeout."""
    try:
        return analyze_crate(crate_name, target_dir)
    except Exception as e:
        print(f"  [ERROR] {crate_name}: {e}", file=sys.stderr)
        return PerCrateAnalysis(
            crate_name=crate_name, total_errors=0, own_errors=0,
            inherited_errors=0, build_status="exception",
            error_codes={}, await_suggestions_count=0, await_fraction=0.0
        )

def analyze_crate(crate_name: str, target_dir: Path) -> PerCrateAnalysis:
    """Run cargo check and analyze."""
    cmd = ["cargo", "check", "-p", crate_name, "--lib", "--message-format=json"]
    env = dict(os.environ, CARGO_TARGET_DIR=str(target_dir))
    try:
        proc = subprocess.run(
            cmd, cwd=str(REPO), env=env, capture_output=True, text=True, timeout=120
        )
    except subprocess.TimeoutExpired:
        return PerCrateAnalysis(
            crate_name=crate_name, total_errors=0, own_errors=0,
            inherited_errors=0, build_status="timeout",
            error_codes={}, await_suggestions_count=0, await_fraction=0.0
        )

    if proc.returncode not in (0, 101):
        return PerCrateAnalysis(
            crate_name=crate_name, total_errors=0, own_errors=0,
            inherited_errors=0, build_status="failed",
            error_codes={}, await_suggestions_count=0, await_fraction=0.0
        )

    diags = []
    for line in proc.stdout.splitlines():
        if line.strip().startswith("{"):
            try:
                msg = json.loads(line)
                if msg.get("reason") == "compiler-message" and msg.get("message"):
                    diags.append(msg["message"])
            except json.JSONDecodeError:
                pass

    errors = [d for d in diags if d.get("level") == "error"]
    own = sum(1 for e in errors if is_own_error(e))
    inherited = len(errors) - own

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
        crate_name=crate_name, total_errors=len(errors),
        own_errors=own, inherited_errors=inherited, build_status="ok",
        error_codes=dict(codes), await_suggestions_count=await_count,
        await_fraction=await_frac
    )

def partition_batches(analyses):
    """Partition into ≤8 batches by repair cost."""
    def cost(a):
        if a.build_status != "ok" or a.own_errors == 0:
            return 0.0
        await_frac = a.await_fraction
        multiplier = 1.0 - (await_frac * 0.9)
        return a.own_errors * multiplier

    sorted_analyses = sorted(analyses, key=cost, reverse=True)
    batches = [[] for _ in range(8)]
    batch_costs = [0.0] * 8

    for analysis in sorted_analyses:
        c = cost(analysis)
        min_idx = batch_costs.index(min(batch_costs))
        batches[min_idx].append(analysis)
        batch_costs[min_idx] += c

    return [b for b in batches if b]

def main() -> int:
    os.makedirs(TARGET_DIR, exist_ok=True)

    print("🌙️ Luna Fleet Scope [FAST MODE - Parallel]")
    print("=" * 70)

    plugins, extensions = enumerate_fleet_crates()
    all_fleet = plugins + extensions

    print(f"[1] Fleet enumeration: {len(plugins)} plugins + {len(extensions)} extensions = {len(all_fleet)} crates")

    print(f"\n[2] Running cargo check (parallel, max 4 concurrent)...")

    analyses = []
    with concurrent.futures.ThreadPoolExecutor(max_workers=4) as executor:
        futures = {
            executor.submit(analyze_crate_safe, crate, TARGET_DIR): crate
            for crate in all_fleet
        }

        for i, future in enumerate(concurrent.futures.as_completed(futures), 1):
            crate = futures[future]
            result = future.result()
            analyses.append(result)
            print(f"  [{i:2d}/{len(all_fleet)}] {crate:45} own={result.own_errors:5} "
                  f"inherited={result.inherited_errors:5} status={result.build_status}")

    print(f"\n[3] Generating report...")

    # Statistics
    total_own = sum(a.own_errors for a in analyses)
    total_inherited = sum(a.inherited_errors for a in analyses)
    total_errors = sum(a.total_errors for a in analyses)
    crates_ok = sum(1 for a in analyses if a.build_status == "ok")
    crates_failed = sum(1 for a in analyses if a.build_status != "ok")

    all_codes = defaultdict(int)
    for a in analyses:
        for code, count in a.error_codes.items():
            all_codes[code] += count

    total_await = sum(a.await_suggestions_count for a in analyses)
    await_frac_all = total_await / total_own if total_own > 0 else 0.0

    batches = partition_batches(analyses)

    # Write report
    report_path = TICKET / "📓️luna-fleet-scope-report.md"
    with open(report_path, "w", encoding="utf-8") as fh:
        fh.write("# Luna Fleet Scope Report\n\n")
        fh.write(f"**Date:** 2026-08-20  \n**Fleet:** {len(all_fleet)} crates\n\n")

        fh.write("## Summary\n\n")
        fh.write(f"- **Total fleet crates:** {len(all_fleet)} ({len(plugins)} plugins + {len(extensions)} extensions)\n")
        fh.write(f"- **Build status:** {crates_ok} buildable, {crates_failed} blocked\n")
        fh.write(f"- **Total diagnostic errors:** {total_errors:,}\n")
        fh.write(f"- **Own errors:** {total_own:,}\n")
        fh.write(f"- **Inherited errors (🧰️framework/):** {total_inherited:,}\n")
        fh.write(f"- **Await-eligible fraction:** {await_frac_all:.1%}\n\n")

        fh.write("## Top Error Codes (Own Errors)\n\n")
        fh.write("| Code | Count |\n|------|-------|\n")
        for code in sorted(all_codes, key=lambda c: all_codes[c], reverse=True)[:15]:
            fh.write(f"| {code} | {all_codes[code]:,} |\n")
        fh.write("\n")

        fh.write("## Per-Crate Breakdown\n\n")
        fh.write("| Crate | Own | Inherited | Status | Await% |\n")
        fh.write("|-------|-----|-----------|--------|--------|\n")
        for a in sorted(analyses, key=lambda x: x.own_errors, reverse=True):
            frac_str = f"{a.await_fraction:.0%}" if a.own_errors > 0 else "-"
            fh.write(f"| {a.crate_name} | {a.own_errors} | {a.inherited_errors} | {a.build_status} | {frac_str} |\n")
        fh.write("\n")

        fh.write("## Batch Partition (≤8 batches)\n\n")
        for batch_idx, batch in enumerate(batches, 1):
            batch_cost = sum(
                max(0, (1.0 - (a.await_fraction * 0.9)) * a.own_errors)
                for a in batch if a.build_status == "ok"
            )
            total_batch_own = sum(a.own_errors for a in batch)
            fh.write(f"### Batch {batch_idx} (cost: {batch_cost:.0f}, own errors: {total_batch_own})\n\n")
            fh.write("```\n")
            for a in sorted(batch, key=lambda x: x.own_errors, reverse=True):
                frac_str = f"{a.await_fraction:.0%}" if a.own_errors > 0 else "-"
                effort = "Low" if a.await_fraction > 0.7 else (
                    "Med" if a.await_fraction > 0.3 else "High"
                )
                fh.write(f"{a.crate_name:45} own={a.own_errors:5} await={frac_str:>5} {effort}\n")
            fh.write("```\n\n")

    print(f"✓ Report: {report_path}")
    print(f"\n[SUMMARY]")
    print(f"  Fleet: {len(all_fleet)} crates")
    print(f"  Own errors: {total_own:,} | Inherited: {total_inherited:,}")
    print(f"  Await-eligible: {await_frac_all:.1%}")
    print(f"  Batches: {len(batches)}")

    return 0

if __name__ == "__main__":
    sys.exit(main())
