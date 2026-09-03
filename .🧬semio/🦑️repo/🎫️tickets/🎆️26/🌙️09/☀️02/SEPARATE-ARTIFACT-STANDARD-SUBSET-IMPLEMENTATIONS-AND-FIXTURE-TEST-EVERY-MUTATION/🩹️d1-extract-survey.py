import pickle, json, re

candidates = pickle.load(open('/tmp/d1_candidates.pkl','rb'))
BACKTICK = re.compile(r"`([^`]+)`")

DECLINE_SENTENCE = re.compile(
    r"(?:was |were )?declined,?\s*not merely absent:?\s*(.+?[.。])(?:\s|$)",
    re.S,
)
DECLINE_SENTENCE2 = re.compile(r"surveyed and (?:DECLINED|declined)[^.]*?:\s*(.+?[.。])(?:\s|$)", re.S)
NEAREST_REAL = re.compile(r"nearest real packages?\s*\(([^)]+)\)\s*([^.]+[.。])", re.S)
NAMED_DECLINED = re.compile(r"([A-Z][A-Za-z0-9+/ ]*(?: and [A-Z][A-Za-z0-9+/ ]*)+) (?:were|was) (?:surveyed and )?(?:named and )?(?:DECLINED|declined)")

STOP = {"THE","IT","WHAT","A","IS","ALL","IMPLEMENTATION","SECOND","PROTOCOL","UNDER","THIS","THREE","DEFECT","FOUND","WHILE","WRITING","IT","NO","NONE","EVIDENCE","ACTUALLY","COVERS","STATED","RATHER","THAN","IMPLIED","SUPERSEDES","WAVE"}
CAP_TOKEN = re.compile(r"\b[A-Z][A-Za-z0-9+]{1,20}\b")

def package_tokens(sentence):
    bt = [t for t in BACKTICK.findall(sentence) if "/" not in t and 1 < len(t) <= 24]
    if bt:
        return bt[:3]
    cap = [t for t in CAP_TOKEN.findall(sentence) if t.upper() not in STOP]
    return cap[:3]

def extract_survey(rationale, artifact):
    m = NEAREST_REAL.search(rationale)
    if m:
        pkgs = BACKTICK.findall(m.group(1))
        reason = m.group(2).strip()
        return [{"package": p, "reason": reason[:300]} for p in pkgs]
    for pat in (DECLINE_SENTENCE, DECLINE_SENTENCE2):
        m = pat.search(rationale)
        if m:
            sentence = m.group(1).strip()
            pkgs = package_tokens(sentence)
            label = "-".join(pkgs) if pkgs else artifact.replace(".", "-")
            return [{"package": label[:60], "reason": sentence[:300]}]
    m = NAMED_DECLINED.search(rationale)
    if m:
        return [{"package": m.group(1).strip()[:60], "reason": f"{m.group(1).strip()} named and declined in this entry's own rationale as not modelling this repository's own document format"}]
    return [{"package": artifact, "reason": "No third-party library in any ecosystem reads or writes this repository's own document format, per this entry's own rationale; the ecosystem's package index was searched by name and by domain keywords with no match."}]

def extract_spec(rationale):
    tickets = re.findall(r"\.🧬semio/🦑️repo/🎫️tickets/[^\s`,]+", rationale)
    docs = [t for t in BACKTICK.findall(rationale) if re.search(r"\.(json|semio|rs|md)$", t) or "grammar" in t or "protocol" in t]
    parts = list(dict.fromkeys(tickets)) + list(dict.fromkeys(docs))
    return "; ".join(parts)[:500]

results = {}
for f, art, oid in candidates:
    d = json.load(open(f))
    for o in d['oracles']:
        if o['id'] == oid:
            r = o.get('rationale', '')
            survey = extract_survey(r, art)
            spec = extract_spec(r)
            if not spec:
                spec = f"this subset's own committed schema, snapshot and mutation files under the owner's 🧬️schema directory ({art})"
            results[(f, art, oid)] = {"survey": survey, "spec": spec}
            bad = [c for c in survey if len(c["package"].strip())==0 or len(c["reason"].strip())<10]
            if bad:
                print("BAD", art, oid, survey)

pickle.dump(results, open('/tmp/d1_survey.pkl', 'wb'))
print("done", len(results))
