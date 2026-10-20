"""⚖️ Churn-weighted expected rebuild per value-family edit for the carve stages P0 to P4 (reads sim3.json and churn.json; new crates inherit the churn of the code they come from). Run with `cwd` = the generated folder."""
import json
t = json.load(open("sim3.json")); ch = json.load(open("churn.json"))
OLD = {"control-contract": "value-control-contract", "allocation-return": "value-allocation-return", "retirement": "value-retirement", "shared-issuance": "value-shared-issuance", "native-encoding": "value-native-encoding", "native-decoding": "value-native-decoding", "intrinsic(ABI)": "value-intrinsic", "clone-source": "value-clone-source", "retained-clone": "value-retained-clone", "paged-list": "value-paged-list", "ordered": "value-ordered", "persistent": "value-persistent", "native-owned": "value-native-owned", "factory": "value-factory", "edit": "value-edit", "registry": "value-registry", "schema": "value-schema", "retained-map": "value-retained-map", "paged-text": "value-paged-text", "snapshot-projection": "value-snapshot-projection", "original-aliases": "value-original-aliases", "retirement-queue": "value-retirement-queue", "derive-expansion": "value-derive-expansion"}
W = {k: ch["semio-framework-" + v] for k, v in OLD.items()}
def mean(i):
    num = den = 0
    for k, w in W.items():
        num += w * t[k][i]; den += w
    if i < 4:
        for k, v in {"old derive": "value-derive", "FACADE": "value"}.items():
            w = ch["semio-framework-" + v]; num += w * t[k][i]; den += w
    else:
        for k, w in {"dcodec": 12, "dcanon": 3, "dretire": 6, "dretained": 6, "dres": 4, "TX paged/utf8": 10, "TY type": 5, "intrinsic(ABI)": 20}.items():
            num += w * t[k][i]; den += w
    return round(num / den), den
for i, s in enumerate(["P0 now", "P1 hygiene", "P2 internal dead edges", "P3 ordered wire", "P4 tiers + derive split"]):
    print(s, *mean(i))
