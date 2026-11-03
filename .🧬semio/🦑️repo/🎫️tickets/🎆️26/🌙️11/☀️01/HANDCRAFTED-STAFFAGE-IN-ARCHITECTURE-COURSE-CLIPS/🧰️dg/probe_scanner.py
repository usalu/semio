import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).parent))
import check_style as cs

sample = """/** 🧪 ok doc */
export function f(a: string) {
  // inside body
  const u = `x ${a + "}"} // not a comment`;
  return u; /* trailing */
}
const table = { // inside literal
  k: 1,
};
// top level note
"""
for c in cs.ts_scan(sample):
    print(c)
