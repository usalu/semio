#!/bin/zsh
# 🪜️ Writes the repair ops of `stepladder` (ops/stepladder.json) from the catalog file before the repair: the outlines are taken from the body solids with outline.ts.
source "${0:A:h}/../env.sh"
O="$W/ops/stepladder.json"; rm -f "$O"
cd "$S/📦️packages/🟦️typescript" || exit 1
ol() { timeout 600 "$B" "$W/outline.ts" stepladder "$1" "$2" "$O" --base "$O" "${@:3}" || exit 1 }
ol 50 front "hinge-l=hinge-bolt-l" "hinge-r=hinge-bolt-r" "rail-cap-l=rail-cap-l@after=rear-rail-l,z=2.05" "rail-cap-r=rail-cap-r@after=rear-rail-r,z=2.05"
ol 50 plan "rear-rail-l=rail-l" "rear-rail-r=rail-r" "rail-cap-l=rail-cap-l@after=rear-rail-l,z=2.05" "rail-cap-r=rail-cap-r@after=rear-rail-r,z=2.05" "hinge-l=hinge-bolt-l@after=front-rail-l,z=4.2" "hinge-r=hinge-bolt-r@after=front-rail-r,z=4.2" "platform-axle=platform-axle@after=rear-rail-r,z=3.4" "rest-tube=rest-tube@after=rear-rail-r,z=2.1"
ol 50 plan "tread-1=tread-1" "tread-2=tread-2" "tread-3=tread-3" "tread-4=tread-4"
ol 50 plan "front-rail-l=stile-l" "front-rail-r=stile-r"
ol 100 plan "rest-tube=rest-tube@after=brace-2,like=brace-2,z=2.1" "grab-bar=grab-bar" "hinge-l=hinge-l@after=front-rail-l,z=4.2,pen=0.18" "hinge-r=hinge-r@after=front-rail-r,z=4.2,pen=0.18"
ol 500 front "tread-2=tread-2@after=tread-1,like=tread-1" "tread-4=tread-4@after=tread-3,like=tread-3"
python3 "$W/texts.py" stepladder '[
["50","omitted","the rest tube in the front view and the plan","the rest tube in the front view","das Auflagerohr in Vorderansicht und Draufsicht","das Auflagerohr in der Vorderansicht"],
["50","thinElements","the hinge bolt heads are 0.16 mm strokes in the front view","the hinge bolts with their heads are drawn by their outline in the front view and the plan, as are the rail caps, the platform axle and the rest tube in the plan","die Gelenkbolzenköpfe sind in der Vorderansicht 0,16-mm-Striche","die Gelenkbolzen mit ihren Köpfen sind in Vorderansicht und Draufsicht mit ihrem Umriss gezeichnet, ebenso Hinterholmkappen, Plattformachse und Auflagerohr in der Draufsicht"],
["100","omitted","rest tube, hinge and spreaders in the front view","rest tube, hinge and spreaders in the front view; the plan shows rest tube and hinge","Auflagerohr, Gelenk und Spreizen in der Vorderansicht","Auflagerohr, Gelenk und Spreizen in der Vorderansicht; die Draufsicht zeigt Auflagerohr und Gelenk"],
["500","merged","Alternating treads give two separated marks across the full print band.","The side elevation shows alternating treads as two separated marks; the front view shows all four treads as the body has them.","Zwei sichtbare Wechselstufen bleiben im ganzen Druckbereich getrennt.","Die Seitenansicht zeigt Wechselstufen als zwei getrennte Marken; die Vorderansicht zeigt alle vier Stufen wie der Körper."]
]'
