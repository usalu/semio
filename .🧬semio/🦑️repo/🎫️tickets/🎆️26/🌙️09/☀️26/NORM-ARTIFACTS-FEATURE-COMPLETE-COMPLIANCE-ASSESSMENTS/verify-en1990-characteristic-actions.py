"""Check the EN 1990 characteristic-action design case independently with SciPy."""
import json
import math
from pathlib import Path
import subprocess
from scipy import stats


ticket = Path(__file__).resolve().parent
workspace = next(path.parent for path in ticket.parents if path.name == ".🧬semio")
norm = workspace / "✏️s/🔌️plugins/📕️norm"
a = next((norm / "🗿️artifacts/⚖️en1990").glob("🏅*/🔖*/🪆*/✳*"))
checks = a / "🧬️schema/💡️inferences/⚖️checks"
catalogue = (checks / "📇️catalogue/🦀️.rs").read_text()
distributions = (checks / "📈️reliability-math/🦀️.rs").read_text()
fixtures = json.loads((a / "🧪️tests/🧫️fixtures/📇️catalogue/characteristic-actions.json").read_text())
normal = norm / "⚖️compliance/📊️normal-distribution/🦀️.rs"
work = ticket / "🗑️generated"
work.mkdir(exist_ok=True)
probe = work / "en1990-characteristic-actions"
source = probe.with_suffix(".rs")
source.write_text(
    f'#[path = "{normal}"]\nmod normal;\nuse normal::{{phi_inv,phi_cdf,phi_pdf}};\n'
    + distributions[distributions.index("const EULER_GAMMA"):distributions.index("//#endregion 🔖️Distributions")]
    + r"""
use std::{collections::BTreeMap, io::{self, BufRead}};
#[derive(Clone, Copy)]
pub enum QuantityKind { Force }
struct Outcome { key: &'static str, value: f64 }
impl Outcome {
  fn info(key: &'static str, _: &str, _: &str, _: QuantityKind, value:f64, _:String) -> Self { Self { key,value } }
}
type R = Result<Vec<Outcome>,String>;
struct Case { numbers: BTreeMap<String, f64>, texts: BTreeMap<String,String> }
impl Case {
    fn n(&self,k:&str)->Result<f64,String> { self.numbers.get(k).copied().ok_or_else(|| k.into()) }
    fn o(&self,k:&str,d:f64)->f64 { self.n(k).unwrap_or(d) }
    fn so<'a>(&'a self,k:&str,d:&'a str)->&'a str { self.texts.get(k).map_or(d,String::as_str) }
}
fn f(value:f64)->String { value.to_string() }
"""
    + catalogue[catalogue.index("fn e90_08("):catalogue.index("fn e90_09(")]
    + r"""
fn main() {
  for line in io::stdin().lock().lines() {
    let line = line.unwrap();
    let mut data = Case { numbers: BTreeMap::new(), texts:BTreeMap::new() };
    for field in line.split('\t') {
      let (key,value) = field.split_once('=').unwrap();
      if let Ok(number) = value.parse() {
        data.numbers.insert(key.into(),number);
      } else {
        data.texts.insert(key.into(),value.into());
      }
    }
    match e90_08(&data) {
      Ok(out) => println!("{}",out.iter().map(|o| format!("{}={:.17e}",o.key,o.value)).collect::<Vec<_>>().join("\t")),
      Err(_) => println!("invalid"),
    }
  }
}
"""
)
subprocess.run(
    ["repo","tools","exec","--light","rustup","--","run","stable","rustc","--edition","2024",str(source),"-o",str(probe)],
    check=True, cwd=workspace,
)
inputs = "".join("\t".join(f"{key}={value}" for key,value in {**row["num"],**row["text"]}.items())+"\n" for row in fixtures)
results = subprocess.check_output([probe], input=inputs, text=True).splitlines()
assert len(results) == len(fixtures)
for row,result in zip(fixtures,results):
    name = row["id"]
    if not row["valid"]:
        assert result=="invalid", (name,result)
        continue
    n, text = row["num"], row["text"]
    expected = {}
    if text["g_value_basis"] == "single-mean":
        expected["g-k"] = n["g_mean"]
    if text["g_value_basis"] == "normal-bounds":
        mean, sd = n["g_mean"], abs(n["g_mean"])*n["v_g"]
        expected["g-k-sup"] = stats.norm.ppf(.95,loc=mean,scale=sd) if sd else mean
        expected["g-k-inf"] = stats.norm.ppf(.05,loc=mean,scale=sd) if sd else mean
    if text["g_value_basis"] == "prescribed-bounds":
        expected.update({"g-k-sup":n["g_prescribed_upper"], "g-k-inf":n["g_prescribed_lower"]})
    if text["q_value_basis"] == "prescribed":
        expected["q-k"] = n["q_prescribed"]
    if text["q_value_basis"] == "annual-gumbel":
        scale = n["q_mean"]*n["q_cov"]/stats.gumbel_r.std()
        location = n["q_mean"] - scale*stats.gumbel_r.mean()
        expected["q-k"] = stats.gumbel_r.ppf(n.get("q_probability",.98), loc=location, scale=scale) if scale else n["q_mean"]
    reported = dict(value.split("=") for value in result.split("\t"))
    assert set(reported)==set(expected), (name,reported,expected)
    assert all(math.isclose(float(reported[key]), target, rel_tol=1e-10, abs_tol=1e-10) for key,target in expected.items()), (name,reported,expected)
print(f"[DEBUG] The {len(fixtures)} EN 1990 characteristic-action cases matched Rust and SciPy.")
