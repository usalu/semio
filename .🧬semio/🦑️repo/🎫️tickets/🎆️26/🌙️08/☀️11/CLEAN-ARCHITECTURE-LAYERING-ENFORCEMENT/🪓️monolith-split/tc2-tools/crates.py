import os,sys,csv
sys.path.insert(0,os.path.dirname(__file__))
import testonly as T
ROOT="/Users/ueli/Documents/semio/"
def manifest_dirs():
    rows=csv.DictReader(open(T.base+"/tc2-census-crates.tsv"),delimiter="\t")
    return {r['crate'].replace("semio-s-artifact-",""):os.path.dirname(ROOT+r['manifest']) for r in rows}
def test_files(short, kinds=("source",)):
    return [ROOT+r[3] for r in T.testonly(short) if r[0] in kinds]
