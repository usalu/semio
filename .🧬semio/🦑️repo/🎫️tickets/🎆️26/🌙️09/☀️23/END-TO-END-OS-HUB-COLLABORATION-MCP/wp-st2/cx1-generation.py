#!/usr/bin/env python3
"""🧮️ CX1 window-3 helper: computes the `🧬️stdio-gis-bootstrap` fixture's profile generation id from a tree's CURRENT
receipts + fixture profile with the tree's OWN hub script (`projectTrustedBootstrapCodecsV1` +
`trustedBootstrapProfileEncoding`, exported only in a transient copy beside it, removed afterwards), and writes it to
`cx1-payload/bootstrap-generation.txt` so `cx1-apply.py` replaces the four stale ids. Run after the other CX1 hunks are
applied to <root> (the codec counts in the profile are part of the encoding).

usage: python3 cx1-generation.py <root> [--write-payload]
"""
import json, os, subprocess, sys

root = sys.argv[1]
hub = os.path.join(root, "🌎️hub/📦️packages/🦀️rust")
source = open(os.path.join(hub, "📜️script.ts"), encoding="utf-8").read()
for name in ("function trustedBootstrapProfileEncoding(", "function projectTrustedBootstrapCodecsV1("):
    assert source.count("\n" + name) == 1, name
    source = source.replace("\n" + name, "\nexport " + name)
copy, runner = os.path.join(hub, "st2probe-script.ts"), os.path.join(hub, "st2probe-run.ts")
try:
    open(copy, "w", encoding="utf-8").write(source)
    open(runner, "w", encoding="utf-8").write('''import { readFileSync } from "node:fs";
import { createHash } from "node:crypto";
import { join } from "node:path";
import { projectTrustedBootstrapCodecsV1, trustedBootstrapProfileEncoding } from "./st2probe-script.ts";
const repoRoot = process.argv[2];
const fixture = JSON.parse(readFileSync(join(repoRoot, "🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🧫️fixtures/🧬️stdio-gis-bootstrap/🔣️.json"), "utf8"));
const codecs = projectTrustedBootstrapCodecsV1(JSON.parse(readFileSync(join(repoRoot, fixture.sources.stdioReceipts), "utf8")), JSON.parse(readFileSync(join(repoRoot, fixture.sources.gisReceipts), "utf8"))).codecs;
console.log(JSON.stringify({ stdio: codecs.stdio.length, gis: codecs.gis.length, fixtureGeneration: fixture.profile.generationId, generation: createHash("sha256").update(trustedBootstrapProfileEncoding(fixture.profile, codecs)).digest("hex") }));
''')
    result = subprocess.run(["bun", runner, root], cwd=hub, capture_output=True, text=True)
finally:
    for path in (copy, runner):
        if os.path.exists(path):
            os.remove(path)
line = next((line for line in result.stdout.splitlines() if line.startswith("{")), None)
if result.returncode != 0 or line is None:
    sys.exit(f"cx1-generation: probe failed rc={result.returncode}: {result.stderr.strip()[-600:]}")
answer = json.loads(line)
print(f"cx1-generation: {line}")
if "--write-payload" in sys.argv:
    open(os.path.join(os.path.dirname(os.path.abspath(__file__)), "cx1-payload", "bootstrap-generation.txt"), "w").write(answer["generation"] + "\n")
    print("cx1-generation: payload written")
