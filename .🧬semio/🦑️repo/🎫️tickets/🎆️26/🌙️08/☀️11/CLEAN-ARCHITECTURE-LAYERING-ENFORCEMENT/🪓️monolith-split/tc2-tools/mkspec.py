import json,sys,os
sys.path.insert(0,os.path.dirname(__file__))
import crates
P="/Users/ueli/Documents/semio/"
def make(short, node, summary, emoji="🥣️", files=None, extra=None, name=None, tag="", folder="🥣️companion"):
    parent="semio-s-artifact-"+short
    files=files if files is not None else [f.replace(P,"") for f in crates.test_files(short)]
    spec=dict(parent=parent,name=(name and "semio-s-artifact-"+name) or parent+"-tests",dir=node+"/🧪️tests/"+folder+tag,files=files,testkit=[parent],emoji=emoji,summary=summary,
      description=f"Companion test crate of {parent}: {summary.rstrip('.')}")
    if extra: spec.update(extra)
    out=os.path.join(os.path.dirname(__file__),"specs",(name or short+"-tests")+".json")
    json.dump(spec,open(out,"w"),ensure_ascii=False,indent=1)
    return spec
if __name__=="__main__":
    D="✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/"
    make("draw-drawing-transform",D+"🔀️transform","the nine mutation fixture laws of the transform leaves.",name="draw-drawing-transform-tests")
    make("draw-drawing-style",D+"🎨️style","the eight mutation fixture laws of the style leaves.",name="draw-drawing-style-tests")
    make("draw-drawing-structure",D+"🧱️structure","the four mutation fixture laws of the structure leaves.",name="draw-drawing-structure-tests")
