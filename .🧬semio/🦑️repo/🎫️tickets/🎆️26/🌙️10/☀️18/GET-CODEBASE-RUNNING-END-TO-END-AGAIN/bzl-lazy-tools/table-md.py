#!/usr/bin/env python3
# Prints the result matrix of matrix-all.sh as a markdown table.
import json, os
G = os.path.join(os.path.dirname(os.path.abspath(__file__)), "..", "🗑️generated", "bzl-lazy")
ORDER = [("fg-generate","framework","@semio-tech/framework-graph:generate"),("fg-test","framework","@semio-tech/framework-graph:test"),("layout-src","s plugin","@semio-tech/layout-layout-rs:test-media-adoption-source"),("writer-test","s plugin","@semio-tech/writer-writer-rs:test"),("writer-build","s plugin","@semio-tech/writer-writer-rs:build"),("ui-styling","framework","@semio-tech/ui-styling:generate"),("hub-test","hub","os-hub-ts:test"),("hub-typecheck","hub","os-hub-ts:typecheck"),("arch-test","teaching","@teaching/architecture-course:test"),("arch-typecheck","teaching","@teaching/architecture-course:typecheck"),("arch-build","teaching","@teaching/architecture-course:build"),("proctor-test","teaching","@teaching/proctor:test"),("mb-bericht","mit-bestand","@semio-tech/mit-bestand-bericht:akteursnetz-validate"),("mb-demo-build","mit-bestand","@semio-tech/mit-bestand-demonstrator:build"),("mb-demo-activate","mit-bestand","@semio-tech/mit-bestand-demonstrator:activate-prototype"),("play-activate","semio-tech","@semio-tech/semio-tech-play:activate-prototype")]
print("| target | area | closure (cached) | input file targets | generate ms (impl+gen) | cold load+analyze ms | warm server wall ms | cold build: actions / executed | warm: executed | after `clean`: executed / hits | after server+output-base loss: executed / hits |")
print("|---|---|---:|---:|---:|---:|---:|---|---:|---|---|")
for n, area, ident in ORDER:
    g = json.load(open(f"{G}/gen-{n}.json")); m = json.load(open(f"{G}/m-{n}.json"))
    ac, aw, ec, ew, cl, sl = m["analysis_cold"], m["analysis_warm"], m["exec_cold"], m["exec_warm"], m["after_clean"], m["after_server_loss"]
    print(f"| `{ident}` | {area} | {g['tasks']} ({g['cached']}) | {ac['configured']} | {g['implementationMs']+g['generateMs']} | {ac.get('loadMs',0)+ac.get('analyzeMs',0)} | {aw['wallMs']} | {ec['actions']} / {ec['executed']} | {ew['executed']} | {cl['executed']} / {cl['hits']} | {sl['executed']} / {sl['hits']} |")
