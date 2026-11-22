"""🌳 Record inspected body, structural and numeric-temperature sheets against their catalog sources."""
from hashlib import sha256
from pathlib import Path
import json
t=Path(__file__).resolve().parent;r=t.parents[6];out=t/'🗑️generated/living-current-views'
notes={
'agave':'Individual lance blades bend from one rosette. At 1:500 the structural blades appear as wide as the outer leaves.',
'apple-tree':'The fine crown has distinct tufts and a forked trunk. At 1:500 the repeating polygonal clusters obscure the branch spacing.',
'bamboo-clump':'Separate leaning culms and leaf sprays are visible. At 1:500 the sprays become broad paddles, especially at the top of the middle culm.',
'date-palm':'The narrow trunk, basal taper and open crown persist. The leaflets need a crown close-up to distinguish them from the central rachis.',
'english-oak':'Low lateral limbs support offset crown clusters. Open spaces between the lower crown and trunk remain visible in body and temperature.',
'hornbeam-hedge':'Six clipped crowns form the hedge. Fine leaf relief and root spacing remain visible, while the 1:500 structural view suppresses most branch forks.',
'hosta-clump':'The blades have swept petioles and a distinct central vein. Fine secondary veins disappear progressively while the leaf envelope remains.',
'ivy-on-trellis':'At fine scales the trellis grid shows behind the growth. At 1:200 and 1:500 the structure loses most cross wires although the physical greenery remains attached.',
'japanese-maple':'The crown reads as stacked horizontal discs at all four scales. The largest coarse clusters have almost planar bottoms. This is a visible foliage defect.',
'lombardy-poplar':'The narrow crown and upright axial stem are consistent across the three modes. The diagnostic view needs a branch close-up at the middle of the crown.',
'london-plane-pollarded':'There is a clear bare bole below the pollard heads. The inner fork follows the timber, and the soft crown sits above the heads across scales.',
'miscanthus-grass':'The inflorescences rise above an arching tuft. At 1:500 the grass blades become broad and lose the separation visible in the fine tuft.',
'multi-stem-birch':'Separate trunks carry drooping crowns with visible voids. The coarse structural mass glyphs are more prominent than the slender distal twigs.',
'scots-pine':'The long bare trunk and discontinuous elevated whorls remain distinct. Fine branches separate visibly from the thermal crown.',
'small-leaved-lime':'The dense oval canopy holds its proportions. Repeated flat-sided clusters are conspicuous on the lower half at 1:500.',
'weeping-willow':'Pendant branches reach the lower canopy. The continuous vertical strips read as a closed curtain, particularly over the centre of the trunk.',
'cargo-bike':'Front and rear wheel sizes, front box and stay geometry read clearly. The 1:500 rack and frame tubes gain conspicuous thickness and the rear wheel loses most spoke detail.',
'city-bicycle':'Step-through frame, distinct rear carrier and wheel contacts are visible. Spokes disappear from most wheel sectors at coarse scales.',
'city-bus':'The roof equipment and passenger glazing are distinguishable. The coarse glass band merges the vertical window divisions.',
'city-car':'The coarse structural scene loses its A/C/D uprights and longitudinal roof rails; the higher-detail car has a continuous roof cage. At 1:500 four solid-looking wheels replace exposed rims.',
'delivery-van':'The box and cab roofline and warm/cold glazing are distinct. At 1:500 the wheel faces are simple discs and the cab cross-members need a closer view.',
'electric-kick-scooter':'The deck and steering stem stay at a consistent angle. Coarse wheels lose visible rim and bearing detail.',
'motor-scooter':'The front fork and rear motor case stay apart and the motor has a distinct thermal field. At 1:500 the handlebar reads as a thick wedge and the front wheel as a solid disc.',
'tram':'Articulated unit, roof collector and running gear read in the same location across modes. The coarse glass band merges most bay divisions.',
'vintage-car-1929':'Fine wire-style wheels, fenders, bonnet and running board establish the period shape. At 1:500 the rims and spokes disappear from the tyres.',
'cantilever-balcony':'At 1:50 and 1:100 open balusters bridge the top and lower rails. At 1:200 they become a solid panel and at 1:500 the infill disappears entirely.',
'curtain-wall-bay':'The spandrel and vision pane share one perimeter frame. The closed opaque diagnostic face needs the shipped alpha when evaluating the glass.',
'etics-brick-wall-12':'The masonry, insulation and render are distinguishable from the fine cut edge. The 1:500 cut face appears as one block.',
'etics-brick-wall-30':'The thick insulation is visible next to the brickwork at fine scales. The 1:500 view loses the layer boundary between core and insulation.',
'external-venetian-blind':'Slats remain visible between the head box and the lower fascia. The coarse structural view omits the individually supported slats.',
'glass-balustrade':'A continuous base channel supports the pane. Physical transparency needs the shaded presentation view while the temperature field picks out glass from its rails.',
'glulam-beam':'The beam silhouette and bearing faces agree across modes. Lamination lines are mainly visible at close range.',
'masonry-wall-with-lintel':'The through opening, jamb depth and lintel above it remain readable. The fine facade obscures the lintel by design.',
'panel-radiator':'At 1:50 the pressed front and top openings are distinct. At 1:500 the structural view reduces the pressed load-bearing panels to an open rectangle.',
'sectional-garage-door':'The raised track and corner bends remain attached. Door leaf divisions are clearest at 1:200 and disappear into the 1:500 outer panel.',
'sliding-door':'The leaf is displaced in its spanning rail; the open width remains clear. Coarse modes suppress the handle.',
'solid-brick-wall-1960':'The fine cut shows masonry courses; coarser scales preserve the wall envelope but lose coursing.',
'spiral-stair':'The helical handrail, radial treads and centre support remain aligned at all scales.',
'stair-flight':'Tread rhythm and the support beneath remain aligned. The low oblique camera foreshortens individual risers.',
'steel-column':'Shaft, head plate and base plate are aligned in all views. Bolt clearances cannot be graded from the full-height view.',
'tied-canopy':'The cable runs from wall head to canopy tip and the temperature field distinguishes the canopy underside. The coarse structural wall connection is reduced to a small anchor.',
'tilt-window':'The structural frame encloses the sash opening and the fine body shows its recess. The diagnostic opaque glass needs a transparent presentation view.',
'wheelchair-ramp':'The sloping deck and posts follow a consistent gradient. The coarse view preserves both visible rail heights.',
'concrete-mixer-truck':'Four axle stations and the drum are visible. The 1:200 and 1:500 structural scenes retain floor contacts beneath missing or severely reduced wheel bodies.',
'diesel-generator':'Roof muffler and frame remain, but the 1:200 and 1:500 intake and exhaust louvre openings become solid rectangular panels.',
'evaporative-humidifier':'The intake opening is on the opposite face. The internal exchange field and supporting frame remain visible.',
'excavator':'The boom and dipper reach the bucket with a distinct articulation. At 1:500 the running tracks lose their opening and inner rollers.',
'expansion-valve':'The sensing bulb and capillary lead into the valve. Fine spring detail disappears into the coarse cap and the connector remains visible in temperature.',
'finned-heat-exchanger':'The fine structural scene retains headers and tube runs. At 1:500 it becomes a closed rectangular block.',
'forklift':'The mast, overhead guard and load remain visible. The coarsest frame has ground contacts below wheels that disappear in its structural view.',
'heat-pump-outdoor-unit':'The fan is visible behind the fine grille; the two coarser views replace it with a flat dark disc.',
'oil-pumpjack':'The walking beam, counterweights, connecting link and motor align across scales. The open trestle remains visible.',
'plate-heat-exchanger':'Four circular ports penetrate the frame in the structure view; the physical shell and its pipes remain distinct.',
'scissor-lift':'The basket rails and folded mechanism are present at fine scale. The coarse physical view merges the crossed links into plates.',
'scroll-compressor':'The head, discharge and service terminals remain on the shell. At coarser scales the isolated mounting feet merge into a block base.',
'solar-thermal-collector':'The collector tilt, rear braces and feet are legible. At 1:500 the bracing thickness increases visibly.',
'sorption-wheel':'The wheel face and sector divisions show on the fine rotor. At coarser scales the internal sector frame is reduced to large radial members.',
'tower-crane':'The original 1:500 mast and boom were closed prisms, and its trolley a solid block. The open truss and rail-carriage revision has separate export and load-path checks.',
'wheel-loader':'The lifting arms, bucket, articulation and heat from the rear power unit remain distinguishable. Small wheel hubs are merged at the coarsest scale.'
}
rows=[]
for p in sorted(r.glob('🎓*/*architecture/*staffage/*catalog/*/*/🔣️.json')):
 name=p.parent.name
 if name not in notes:continue
 raw=p.read_bytes();a=json.loads(raw)['asset'];h=sha256(raw).hexdigest();category=a['category'];files=sorted(p.parent.parent.glob('*/🔣️.json'));i=files.index(p)//4
 key=out/(f'art-{category}-{i+1}.sha');viewed=(key.exists() and h in key.read_text())
 rows.append(f'| {category} | {name} | `{h[:16]}` | '+('Matched' if viewed else 'Revision changed; refresh due')+' | '+notes[name]+' |')
assert len(rows)==len(notes)==59
(t/'digital-large-form-review.md').write_text('# Structures, vehicles and vegetation\n\nThe contact sheets were inspected on November 21 across four scales in physical, surface-temperature and structural views. The source-match column compares the catalog with the inspected sheet. Separate detail studies are necessary where camera angle or opaque diagnostic shading hides an assembly.\n\n| Family | Model | Catalog SHA-256 prefix | Inspected sheet | Visual finding |\n| --- | --- | --- | --- | --- |\n'+'\n'.join(rows)+'\n')
