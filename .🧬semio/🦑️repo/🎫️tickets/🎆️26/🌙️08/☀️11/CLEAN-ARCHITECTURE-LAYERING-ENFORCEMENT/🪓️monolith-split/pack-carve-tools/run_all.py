"""▶️ Regenerates the whole stage tree from the pristine base."""
import shutil, sys
from lib import *
from owners import *
from crates import *
from rules import Rules
import carve_format, carve_record as CR, carve_rest, manifests

def main():
    shutil.rmtree(STAGE, ignore_errors=True)
    X = CR.split_pieces()
    om = CR.record_owner_map(X)
    R = Rules({"format": format_owners(), "record": om})
    carve_format.run(R)
    for key in ("format", "retained", "catalog"):
        done = finalize_crate(libroot(key), lambda t, k=key: R.rewrite(t, k))
        print("finalize", key, [d.split("/")[-3:] for d in done])
    CR.build_schema(X, om, R)
    CR.build_rretained(X, om, R)
    CR.build_encode(X, om, R)
    CR.build_preflight(X, om, R)
    CR.build_record(X, om, R)
    for key in ("schema", "rretained", "encode", "preflight", "record"):
        done = finalize_crate(libroot(key), lambda t, k=key: R.rewrite(t, k))
        print("finalize", key, [d.split("/")[-3:] for d in done])
    carve_rest.run(R, X, om)
    manifests.run()
    manifests.run_facade()
    print("stage regenerated")

if __name__ == "__main__":
    main()
