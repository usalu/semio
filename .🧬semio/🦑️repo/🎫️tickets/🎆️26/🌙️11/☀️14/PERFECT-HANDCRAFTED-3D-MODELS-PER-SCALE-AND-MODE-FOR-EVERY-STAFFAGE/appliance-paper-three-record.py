"""📝 Records the complete verified paper ranges and the preserved digital-source fingerprints."""
import json,hashlib
from pathlib import Path
t=Path(__file__).resolve().parent;gen=t/'🗑️generated'/'rotor';r=next(t.parents[6].glob('🎓️teaching/*/🧍️staffage/🗂️catalog/*appliances'))
lines=['# Diffuser, fan-heater and MVHR paper ranges','','All three office-laser drawing bands were checked at every integer denominator from 1:20 through 1:350. At 1:75 and 1:150 both adjoining authored representations were tested. The four digital volume scales in each asset exactly match the pre-edit geometry fingerprint and ticket authoring document.','','| Asset | Resolved paper drawing | Drawing checks | Full product findings | Digital volume SHA-256 |','|---|---|---:|---:|---|']
details={'ceiling-diffuser':'The 7.5 mm backplate edge in both side views uses a 0.20 mm pen.','electric-fan-heater':'The fine side view draws the lower return lip at the measured cable entry. The 6 mm mounting bosses use a 0.20 mm pen, resolving the isolated 1:56 failure. The intermediate side and plan views draw the 2.5 mm edge of the outlet grille with a 0.20 mm pen.','mvhr-unit':'The 1:100 side view marks each EPDM sealing bead by the 100 mm duct connection it seats on. Four separate sleeve lips retain their original centerlines and 150 mm center spacing. The two projected front-view spigots use a 0.13 mm pen to keep the 42 mm gap readable.'}
for slug,original in json.loads((gen/'paper-three-volumes.json').read_bytes()).items():
    raw=next((r/slug).iterdir()).read_bytes();a=json.loads(raw)['asset'];volume=a['volumes']
    assert hashlib.sha256(json.dumps(volume,sort_keys=True,separators=(',',':')).encode()).hexdigest()==original
    assert json.loads((t/'volumes'/'appliances'/(slug+'.json')).read_bytes())['volumes']==volume
    strict=json.loads((gen/(slug+'-live-strict.json')).read_bytes());assert not strict['issues'] and strict['validateAsset']==strict['volumes']==strict['schema']==0
    rows=[json.loads(s)for s in (gen/(slug+'-all-paper.jsonl')).read_text().splitlines()];assert len(rows)==333 and all(q['suitable']for q in rows),(slug,[q for q in rows if not q['suitable']][:5])
    assert a['representations']['500']['status']=='unavailable'
    lines.append(f"| {slug} | {details[slug]} | {len(rows)} | 0 | `{original}` |")
lines+=['','The product validator also checks asset schema, all available 3D representations, thermal coverage and structural geometry. Run `rotor-paper.ts` and `r3-validate.ts --file <asset> --volumes --json` against the live catalog documents to reproduce the checks.','']
(t/'appliance-paper-three.md').write_text('\n'.join(lines))
