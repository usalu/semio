"""🔲 Compare the production boundary shortcut directly against GEOS triangle unions."""
from pathlib import Path
import json
import random
import subprocess
import sys
from shapely.geometry import Polygon
from shapely.ops import unary_union

ticket = Path(__file__).resolve().parent
work = ticket / '🗑️generated'
source = next(Path('✏️s/🔌️plugins/🏙️bim').rglob('🔲️outline/🦀️.rs')).resolve()
harness = work / 'outline.rs'
harness.write_text('#[path = ' + json.dumps(str(source),ensure_ascii=False) + '] mod outline;\n' + '''
use std::io::{self, BufRead};
fn main() {
    for line in io::stdin().lock().lines() {
        let input: Vec<f64> = line.unwrap().split_whitespace().map(|x| x.parse().unwrap()).collect();
        let triangles = input.chunks_exact(6).map(|p| [[p[0],p[1]],[p[2],p[3]],[p[4],p[5]]]);
        match outline::single_front_outline(triangles) {
            Some(ring) => println!("{:?}", ring),
            None => println!("null"),
        }
    }
}
''')
binary = work/'outline-oracle'
subprocess.run(['rustc','--edition=2024',str(harness),'-o',str(binary)],check=True)
rng = random.Random(9971)
cases = []
for _ in range(150):
    cells = [(x,y) for x in range(5) for y in range(4) if rng.random() < rng.choice([0.35,0.75,1.0])]
    triangles = []
    for x, y in cells:
        a,b,c,d = (x,y),(x+1,y),(x+1,y+1),(x,y+1)
        triangles.extend([(a,b,c),(a,c,d)] if rng.randrange(2) else [(a,b,d),(b,c,d)])
    if triangles:
        rng.shuffle(triangles)
        cases.append(triangles)
for _ in range(150):
    triangles = []
    for _ in range(rng.randrange(1,8)):
        tri = [tuple(rng.choices(range(-3,4),k=2)) for _ in range(3)]
        if Polygon(tri).area:
            triangles.append(tri)
    if triangles:
        cases.append(triangles)
def rows(triangles):
    return ' '.join(str(x) for triangle in triangles for point in triangle for x in point)
output = subprocess.run([str(binary)],input='\n'.join(map(rows,cases))+'\n',text=True,capture_output=True,check=True)
admitted=0
for triangles,line in zip(cases,output.stdout.splitlines(),strict=True):
    ring=json.loads(line)
    if ring:
        shape=Polygon(ring)
        assert shape.is_valid and shape.symmetric_difference(unary_union([Polygon(tri) for tri in triangles])).area < 1e-10, (triangles,ring)
        admitted+=1
assert admitted > 20
print(f'{len(cases)} GEOS comparisons, {admitted} certified simple silhouettes')
