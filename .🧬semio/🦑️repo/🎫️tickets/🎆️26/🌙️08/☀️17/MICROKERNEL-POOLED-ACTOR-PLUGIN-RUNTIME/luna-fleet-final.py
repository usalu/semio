#!/usr/bin/env python3
"""🌙️ Luna Fleet Scope - Final Report using known data + sampling."""

import json
import re
import subprocess
import sys
from collections import defaultdict
from concurrent.futures import ThreadPoolExecutor, as_completed
from pathlib import Path
from dataclasses import dataclass

REPO = Path("/Users/ueli/Documents/semio")
TICKET = REPO / ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME"
TARGET_DIR = Path("/private/tmp/claude-501/-Users-ueli-Documents-semio/e6a44461-bab7-421f-8a53-65123a5e9482/scratchpad/target-luna")

@dataclass
class Result:
    name: str
    total: int
    own: int
    inherited: int
    status: str
    await_pct: float

def enumerate_fleet():
    """Get list of all fleet crates."""
    def _extract_name(path):
        try:
            with open(path, "r") as fh:
                m = re.search(r'name\s*=\s*["\']([^"\']+)["\']', fh.read())
                return m.group(1) if m else None
        except:
            return None

    with open(REPO / "Cargo.toml", "r") as fh:
        members_match = re.search(r'\[workspace\].*?members\s*=\s*\[(.*?)\]', fh.read(), re.DOTALL)
        member_paths = [m.group(1) for m in re.finditer(r'"([^"]+)"', members_match.group(1))]

    crates = []
    for path in member_paths:
        cargo_path = REPO / path / "Cargo.toml"
        if not cargo_path.exists():
            continue
        name = _extract_name(cargo_path)
        if not name:
            continue
        if name.startswith("semio-s-plugin-") or "extension" in name.lower() or "extensio" in path.lower():
            crates.append(name)
    return sorted(crates)

def analyze_fanout(crate_name):
    """Use pre-generated fanout file if available."""
    fanout_file = TICKET / f"fanout-{crate_name}.json"
    if not fanout_file.exists():
        return None

    def is_own(d):
        for sp in d.get("spans", []):
            if sp.get("is_primary"):
                return "🧰️framework/" not in sp.get("file_name", "")
        return False

    def count_await(d):
        c = 0
        def walk(n):
            nonlocal c
            for ch in n.get("children", []):
                for sp in ch.get("spans", []):
                    if "await" in (sp.get("suggested_replacement", "") or "").lower():
                        c += 1
                walk(ch)
        walk(d)
        return c

    diags = []
    try:
        with open(fanout_file, "r") as fh:
            for line in fh:
                if line.strip().startswith("{"):
                    try:
                        msg = json.loads(line)
                        if msg.get("reason") == "compiler-message":
                            diags.append(msg.get("message"))
                    except:
                        pass
    except:
        return None

    errors = [d for d in diags if d and d.get("level") == "error"]
    own = sum(1 for e in errors if is_own(e))
    inherited = len(errors) - own
    await_count = sum(count_await(e) for e in errors if is_own(e))
    await_pct = 100.0 * await_count / own if own > 0 else 0

    return Result(crate_name, len(errors), own, inherited, "ok", await_pct)

def quick_check(crate_name):
    """Quick cargo check with 15s timeout for sampling."""
    cmd = ["cargo", "check", "-p", crate_name, "--lib", "--message-format=json"]
    env = dict(__import__('os').environ, CARGO_TARGET_DIR=str(TARGET_DIR))
    try:
        proc = subprocess.run(cmd, cwd=str(REPO), env=env, capture_output=True,
                             text=True, timeout=15)
        if proc.returncode not in (0, 101):
            return Result(crate_name, 0, 0, 0, "failed", 0)

        diags = []
        for line in proc.stdout.splitlines():
            if line.strip().startswith("{"):
                try:
                    msg = json.loads(line)
                    if msg.get("reason") == "compiler-message":
                        diags.append(msg.get("message"))
                except:
                    pass

        errors = [d for d in diags if d and d.get("level") == "error"]
        if not errors:
            return Result(crate_name, 0, 0, 0, "ok", 0)

        def is_own(d):
            for sp in d.get("spans", []):
                if sp.get("is_primary"):
                    return "🧰️framework/" not in sp.get("file_name", "")
            return False

        def count_await(d):
            c = 0
            def walk(n):
                nonlocal c
                for ch in n.get("children", []):
                    for sp in ch.get("spans", []):
                        if "await" in (sp.get("suggested_replacement", "") or "").lower():
                            c += 1
                    walk(ch)
            walk(d)
            return c

        own = sum(1 for e in errors if is_own(e))
        inherited = len(errors) - own
        await_count = sum(count_await(e) for e in errors if is_own(e))
        await_pct = 100.0 * await_count / own if own > 0 else 0

        return Result(crate_name, len(errors), own, inherited, "ok", await_pct)
    except subprocess.TimeoutExpired:
        return Result(crate_name, 0, 0, 0, "timeout", 0)
    except Exception as e:
        return Result(crate_name, 0, 0, 0, f"error: {e}", 0)

def main():
    print("🌙️ Luna Fleet Scope - Final Analysis")
    print("=" * 70)

    crates = enumerate_fleet()
    print(f"Fleet: {len(crates)} crates\n")

    # Try fanout first (pre-analyzed), then quick check
    results = []
    print("Analyzing...")
    with ThreadPoolExecutor(max_workers=6) as executor:
        futures = {}
        for crate in crates:
            # Try fanout first
            r = analyze_fanout(crate)
            if r:
                results.append(r)
                print(f"  [fanout] {crate:45} own={r.own:6} await={r.await_pct:5.1f}%")
            else:
                # Queue quick check
                futures[executor.submit(quick_check, crate)] = crate

        for future in as_completed(futures):
            crate = futures[future]
            try:
                r = future.result()
                results.append(r)
                print(f"  [check]  {crate:45} own={r.own:6} await={r.await_pct:5.1f}% ({r.status})")
            except Exception as e:
                print(f"  [error]  {crate:45} {e}")

    print(f"\n[Results] {len(results)} crates analyzed")

    # Stats
    total_own = sum(r.own for r in results)
    total_inherited = sum(r.inherited for r in results)
    total_errors = sum(r.total for r in results)
    await_total = sum(r.own * r.await_pct / 100.0 for r in results)
    await_frac = await_total / total_own if total_own > 0 else 0

    results_ok = [r for r in results if r.status == "ok"]
    print(f"\n[Summary]")
    print(f"  Total errors: {total_errors:,}")
    print(f"  Own errors: {total_own:,}")
    print(f"  Inherited errors: {total_inherited:,}")
    print(f"  Crates analyzed: {len(results_ok)}/{len(results)}")
    print(f"  Await-eligible: {await_frac:.1%}")

    # Partition
    def cost(r):
        if r.own == 0:
            return 0
        mult = 1.0 - (r.await_pct / 100.0) * 0.9
        return r.own * mult

    sorted_r = sorted(results_ok, key=cost, reverse=True)
    batches = [[] for _ in range(8)]
    batch_costs = [0.0] * 8
    for r in sorted_r:
        idx = batch_costs.index(min(batch_costs))
        batches[idx].append(r)
        batch_costs[idx] += cost(r)

    batches = [b for b in batches if b]

    # Write report
    report = TICKET / "📓️luna-fleet-scope-report.md"
    with open(report, "w") as fh:
        fh.write("# Luna Fleet Scope Report\n\n")
        fh.write(f"**Fleet:** {len(crates)} crates  \n")
        fh.write(f"**Analyzed:** {len(results_ok)} OK, {len(results)-len(results_ok)} blocked\n\n")

        fh.write("## Summary\n\n")
        fh.write(f"- **Total errors:** {total_errors:,}\n")
        fh.write(f"- **Own errors:** {total_own:,}\n")
        fh.write(f"- **Inherited errors:** {total_inherited:,}\n")
        fh.write(f"- **Await-eligible:** {await_frac:.1%}\n\n")

        fh.write("## Per-Crate (Top 20 by own errors)\n\n")
        fh.write("| Crate | Total | Own | Inherited | Status | Await% |\n")
        fh.write("|-------|-------|-----|-----------|--------|--------|\n")
        for r in sorted(results, key=lambda x: x.own, reverse=True)[:20]:
            fh.write(f"| {r.name} | {r.total:,} | {r.own:,} | {r.inherited:,} | {r.status} | {r.await_pct:.0f}% |\n")
        fh.write("\n")

        fh.write("## Proposed Batches\n\n")
        for i, batch in enumerate(batches, 1):
            batch_cost = sum(cost(r) for r in batch)
            batch_own = sum(r.own for r in batch)
            fh.write(f"### Batch {i}\n**Cost: {batch_cost:.0f} | Own errors: {batch_own:,}**\n\n")
            for r in sorted(batch, key=lambda x: x.own, reverse=True):
                effort = "🟢" if r.await_pct > 70 else ("🟡" if r.await_pct > 30 else "🔴")
                fh.write(f"- {r.name:40} own={r.own:6,} await={r.await_pct:5.1f}% {effort}\n")
            fh.write("\n")

    print(f"\n✓ Report: {report}")
    print(f"✓ Batches: {len(batches)}")

if __name__ == "__main__":
    main()
