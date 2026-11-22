# Mallard webbed paddles, leg attachment and heat exchange

The catalog revision `df195859fe` bound the entire coarse foot to the tarsus. At 1:200 and 1:500 the actual foot joint carried zero skin solids. Turning the foot in the swimming poses rotated the skeleton while the visible stroke stayed on the shank. A second gap opened above the hock where the middle of the feathered shank had been removed.

Revision `dbcb5d687f23ca6f262af65fee9d05533a420e42df9c72ec2d74733c9eca5458` has a medial, middle and lateral foretoe with two scalloped webs and a separate, horn-tipped hallux. Both webs have a dorsal bow, a plantar surface and closed edges. Each membrane terminates against the two toe shafts that support it. The four scales keep the paddle width, the edge recesses between toes and the rear offset of the hallux. The broader drumstick now meets the belly during both the power stroke and upending. Its plumage and the exposed shank use their respective heat regions. The skeleton carries four distinct phalangeal rays per foot.

## Posed geometry

For both feet at each of four scales the modeled width was 42.6 mm and the combined plan area of the two membranes was 1,027 mm² per foot. This area excludes the toes themselves. Three.js triangle projection measured the membrane area. Distance to the nearest posed triangle at the plantar support point was at most 0.26 mm. A ray-intersection test found the feathered shank entering the trunk on both sides in all five poses. During swimming and the tested power stroke both paddles lay below the water plane. The feet broke clear of it during upending. The resting pose folds both legs beneath the breast and carries support on the breast and the two plantar webs.

## Modeled heat exchange

The modeled exposed area of both bare feet in standing pose was 0.006749 m². The area was measured over the posed mesh after occlusion. The thermal region uses that value as its exchange area. The contact fractions and boundary temperatures below are modeling assumptions. The temperatures and contact heat flux are solver results for the asset’s reference environment.

| Pose | Contact surface | Modeled contact fraction | Foot surface | Contact heat flux |
|---|---|---:|---:|---:|
| Standing | Dry bank at 18 °C | 45% | 21.3 °C | 30.0 W/m² |
| Swimming | Water at 14 °C | 100% | 26.6 °C | 251.1 W/m² |
| Paddle stroke 3 | Water at 14 °C | 100% | 26.6 °C | 251.1 W/m² |
| Resting ashore | Dry bank at 18 °C | 42% | 25.8 °C | 65.6 W/m² |
| Upending | Air | 0% | 26.1 °C | 0 W/m² |

In the resting pose the breast contacts the bank over 12% of its thermal region and the folded feet expose 20% of theirs to air. Tests distinguish surface exposure, immersion and contact. The solver values describe this model and have no live-animal calibration.

## Verification and visual evidence

One native worker ran the full schema and semantic check, all 13 volume rules at each denominator and an independent glTF reader over 12 exported scenes (2,408,840 bytes). No findings remained. The original [standing sheet](🗑️generated/living-current-views/mallard-duck-standing-all-presentation-shaded-temperature-df195859fe.png) records the missing coarse paddles. The installed-source sheets show [standing](🗑️generated/living-current-views/mallard-duck-standing-all-presentation-shaded-temperature-dbcb5d687f.png), [swimming](🗑️generated/living-current-views/mallard-duck-swimming-all-presentation-shaded-temperature-dbcb5d687f.png), [power stroke](🗑️generated/living-current-views/mallard-duck-paddle-3-all-presentation-shaded-temperature-dbcb5d687f.png), [resting ashore](🗑️generated/living-current-views/mallard-duck-resting-ashore-all-presentation-shaded-temperature-dbcb5d687f.png) and [upending](🗑️generated/living-current-views/mallard-duck-upending-all-presentation-shaded-temperature-dbcb5d687f.png). A [four-scale foot detail](🗑️generated/living-current-views/mallard-duck-paddles-dbcb5d687f.png) shows the dorsal surface, plantar surface, membrane edge and structural rays. Detail colors identify the membrane and toe geometry.

The [language-independent fixture](mallard-paddle-case.json), [triangle and heat checks](mallard-paddle-check.ts), [exchange-area measurement](mallard-paddle-heat.ts), [close-up renderer](mallard-paddle-sheet.ts) and [authoring input](mallard-paddle-fit.ts) are retained. The 1:100–1:500 representations still simplify the head plumage, eye and drake curl. The coarse structural neck remains a thin central line.
