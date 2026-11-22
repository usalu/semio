"""📑 Record independently reopened component and print readbacks against the source hash."""
import hashlib,json,re,sys
from pathlib import Path
from runpy import run_path
T=Path(__file__).resolve().parent;G=T/'🗑️generated'
catalog=run_path(str(T/'digital-building-equipment.py'))['catalog']
entries={
'etics-brick-wall-12':'The end cut shows 15 mm room plaster, a 365 mm bonded brick-and-mortar leaf, 120 mm of wool and 20 mm of render. Four 60 mm anchor plates bear on the insulation. The 1:500 thermal section retains all material interfaces.',
'etics-brick-wall-30':'The end cut shows 15 mm room plaster, a 365 mm bonded brick-and-mortar leaf, 300 mm of wool and 20 mm of render. The wool is visibly wider than on the 120 mm specimen. The four headed restraint anchors remain in the force view.',
'external-venetian-blind':'Each force view now shows eighteen separately pivoted slats between the guides. Both 8 mm lift tapes connect to the head bearing. The reverse view also exposes the bottom rail and guide pins.',
'solar-thermal-collector':'Two bolted 2 × 20 mm straps cross behind the folded foot channels, posts and knee stays. The sheet shows the straps and their bolts from the rear. Inside the folded tray the 3.2 mm cover, absorber, mineral wool and two actual headers remain distinct.',
'tied-canopy':'Both gusset and clevis pairs close around real pins. At the wall, two 20 mm plates and two thermal pads are drilled for four 16 mm studs apiece. The paired studs project through the separate nuts.',
'sectional-garage-door':'Four finger-safe profiled panels roll on ten independent running wheels. Three rows of fitted hinge leaves and paired pins connect the articulated sections. The overhead tracks bear on actual hangers and diagonal braces.',
'masonry-wall-with-lintel':'Thirty-three courses of facing units and mortar run to the jambs. The insulating cavity, inner calcium-silicate blocks, internal plaster returns, separate lintel bearings and sloping sill remain visible in cut section.',
'curtain-wall-bay':'Two separate 6 mm panes enclose a 16 mm argon space. The perimeter seal remains outside that clear opening. Enamel glazing, mineral wool and the 1.5 mm folded steel backpan remain separately built in the spandrel. The coarser exterior thermograms show the enamel surface.',
'tilt-window':'Two separate 4 mm panes surround a clear 16 mm gas cavity and a perimeter seal. The glass and handle travel with the tilting sash. The stay, hinges, sill and stops remain distinct.',
'glass-balustrade':'Two drilled 10 mm glass panes surround 1.5 mm PVB. Seven cross bolts run through the clamping cheeks. Two separate five-stud rows go through the perforated 15 mm base plate.',
'sliding-door':'The 10 mm glass leaf remains framed. Each side has its own stand-off pull handle. Two four-wheel carriages, their hangers and the open overhead track remain visible from behind. The exterior thermal view follows the three exposed faces of the track.',
'solid-brick-wall-1960':'Twelve distinct bed joints, alternating rows of headers and stretchers, separate mortar and two real finish layers continue through the cut edge. The structure view tracks the actual bonded units.',
}
for slug in sys.argv[1:]:
    cat=catalog(slug);a=json.loads(cat.read_bytes())['asset'];sha=hashlib.sha256(cat.read_bytes()).hexdigest()[:16]
    assert 'schema 0, validateAsset 0, checkVolumes 0' in (G/(slug+'-art-canonical.log')).read_text()
    audit=json.loads(next(l for l in (G/(slug+'-art-glb.log')).read_text().splitlines() if l.startswith('{')))
    assert audit['scenes']==12 and audit['findings']==[]
    for variant in ('sheet','sheet-reverse'):assert sha[:10] in (G/(slug+'-art-'+variant+'.log')).read_text()
    body=entries[slug];note=f'{body} All twelve exported scenes reopened ({audit["bytes"]:,} bytes). The strict catalog source readback passed.'
    kind=a['category'];p=T/'digital-component-cut-and-hardware.md';s=p.read_text()
    heading='## Canonical repairs'
    if heading not in s:s=s.rstrip()+'\n\n'+heading+'\n\n| Model | Canonical SHA-256 prefix | Published section or fitting |\n| --- | --- | --- |\n'
    lines=[l for l in s.splitlines() if not l.startswith('| '+slug+' |')];last=max(i for i,l in enumerate(lines) if l.startswith('| '));lines.insert(last+1,'| '+slug+' | `'+sha+'` | '+note+' |');p.write_text('\n'.join(lines)+'\n')
    for n in ('digital-art-index.md','digital-large-form-review.md'):
        p=T/n;s=p.read_text();lines=s.splitlines(True);ids=[i for i,l in enumerate(lines) if l.startswith('| '+kind+' | '+slug+' | ')];assert len(ids)==1
        lines[ids[0]]='| '+kind+' | '+slug+' | `'+sha+'` | '+('Rendered |' if n=='digital-art-index.md' else 'Matched | '+body+' |')+'\n';p.write_text(''.join(lines))
    p=T/('r3-'+kind+'-'+slug+'.md');s=re.sub(r'\n## Canonical component readback, 2026-11-21\n\n[^\n]+\n?', '',p.read_text())
    p.write_text(s.rstrip()+'\n\n## Canonical component readback, 2026-11-21\n\nCanonical source `'+sha+'`. '+note+'\n')
