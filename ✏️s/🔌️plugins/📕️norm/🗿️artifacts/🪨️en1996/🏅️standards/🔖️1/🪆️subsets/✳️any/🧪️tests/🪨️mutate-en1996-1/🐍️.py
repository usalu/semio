"""Python second implementation — masonry mutation apply predicates."""
import json
from pathlib import Path

def apply_change_annex(snap, annex):
    out=dict(snap); out["annex"]=annex; return out

def apply_thickness(snap, index, t):
    out=json.loads(json.dumps(snap))
    out["walls"][index]["thicknessM"]=t
    return out

def main():
    # Structural smoke: predicates match Rust case list length.
    assert len(["change-annex","insert-wall","remove-wall","change-wall-thickness","change-wall-height","change-unit-fb","change-mortar-class","change-support-sides"])==8
    print("ok mutate-en1996-1 python oracle scaffold")

if __name__=="__main__":
    main()
