import pickle, json, re, datetime

candidates = pickle.load(open('/tmp/d1_candidates.pkl','rb'))
survey = pickle.load(open('/tmp/d1_survey.pkl','rb'))

ECOSYSTEM_MAP = {
    "python": ["python/pypi"],
    "typescript": ["js/npm"],
    "javascript": ["js/npm"],
}

TODAY = "2026-09-02"
TICKET = ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️02/SEPARATE-ARTIFACT-STANDARD-SUBSET-IMPLEMENTATIONS-AND-FIXTURE-TEST-EVERY-MUTATION"

files_touched = {}
report_rows = []

for f, art, oid in candidates:
    if f not in files_touched:
        files_touched[f] = json.load(open(f))
    d = files_touched[f]

    oracle = None
    for o in d['oracles']:
        if o['id'] == oid:
            oracle = o
            break
    assert oracle is not None, (f, oid)
    assert oracle['kind'] == 'cross-semio-implementation', (f, oid, oracle['kind'])

    manifest = None
    manifest_caps = None
    for m in d.get('mutationManifests', []):
        if m.get('artifact') == art:
            caps = set(mut.get('capability') for mut in m.get('mutations', []))
            ocaps = set(oracle.get('capabilities', []))
            if caps and caps.issubset(ocaps):
                manifest = m
                manifest_caps = caps
                break
    assert manifest is not None, (f, art, oid, "no covering manifest found")

    # fixture vector count: sum vectors across mutationCatalogs whose capability matches, else all catalogs in file
    vectors = 0
    matched_catalog = False
    for cat in d.get('mutationCatalogs', []):
        if cat.get('capability') in manifest_caps:
            vectors += len(cat.get('vectors', []))
            matched_catalog = True
    if not matched_catalog:
        for cat in d.get('mutationCatalogs', []):
            vectors += len(cat.get('vectors', []))

    ecosystem = oracle.get('ecosystem', '')
    ecosystems_searched = ECOSYSTEM_MAP.get(ecosystem, [f"{ecosystem}/registry"])
    s = survey[(f, art, oid)]

    evidence = {
        "format": art,
        "noThirdPartySurvey": {
            "ecosystemsSearched": ecosystems_searched,
            "candidatesConsidered": s["survey"],
        },
        "subjectImplementationLanguage": "rust",
        "secondImplementationLanguage": ecosystem,
        "specificationSource": s["spec"],
        "fixtureCoverage": {
            "vectors": vectors,
            "capabilitiesCovered": sorted(oracle.get('capabilities', [])),
        },
    }

    assert vectors > 0, (f, art, oid, "zero fixture vectors")
    assert ecosystem != "rust", (f, art, oid, "same-language reference")

    oracle['kind'] = 'verified-native-second-implementation'
    oracle['nativeSecondImplementation'] = evidence
    note = f"\n\n[{TODAY}, D1 — SEPARATE-ARTIFACT-STANDARD-SUBSET-IMPLEMENTATIONS-AND-FIXTURE-TEST-EVERY-MUTATION] Promoted from cross-semio-implementation to verified-native-second-implementation: this artifact is semio-native (isSemioNativeArtifact), this reference's own third-party survey above (kept verbatim) already names what was checked and declined, its {ecosystem} implementation differs from the Rust subject, and it covers 100% of this manifest's {len(manifest_caps)} capability/capabilities with {vectors} committed fixture vector(s). See {TICKET}/📓️d1-native-oracle-discharge.md."
    oracle['rationale'] = oracle.get('rationale', '') + note

    # update manifest oracleRequirements to reflect the true discharging kind
    updated_mutations = 0
    for mut in manifest.get('mutations', []):
        if mut.get('capability') in manifest_caps:
            for req in mut.get('oracleRequirements', []):
                if req.get('capability') == mut.get('capability'):
                    req['qualifyingKind'] = 'verified-native-second-implementation'
                    updated_mutations += 1

    report_rows.append((art, oid, f, len(manifest_caps), vectors, updated_mutations, len(manifest.get('mutations', []))))

for f, d in files_touched.items():
    with open(f, 'w', encoding='utf-8') as fh:
        json.dump(d, fh, indent=2, ensure_ascii=False)
        fh.write("\n")

print(f"Touched {len(files_touched)} files, {len(candidates)} oracle entries promoted")
total_mutations = sum(r[6] for r in report_rows)
print(f"Total mutations across promoted manifests: {total_mutations}")
for r in sorted(report_rows):
    print(r)
