# Period Car Proportion

The Villa Savoye information clip drew the handcrafted 1929 saloon at catalog metres and then dimensioned it with the teaching model's assumed clearance box, 4.6 m by 1.75 m. The catalog plan is longer than it is wide and smaller than that box on both axes, so the arrows did not meet the body.

The period car now stays at catalog metres. The question card dimensions that body and draws the assumed box as its own dashed envelope. The bay close-up uses the same split: the animated assumed width sizes the envelope and its dimension, and the saloon inside is not stretched. Pinch vignettes draw that same rectangle, which is the outline the clearance gap already measured. The swept path uses the catalog track, because the caption describes the area the car's outline covers.

`🚘️car-scale` covers the card with Shapely (drawn body equals catalog metres, body inside the assumed rectangle, areas differ), the rotated clearance rectangle (area unchanged), and the swept half-width (catalog track, and `_driver` assigns that call). Four tests passed under a 4 GiB grant.

A full-tree guard run stalled and was stopped. The clipped scanner `scan-staffage.py` then read 187 architecture clip scripts. Its only hit was the metre measurement `unit=1.0` in this scene. That call now uses the catalog width as its unit and divides the rendered body back to metres. A repeat scan reports no pictogram, primitive-object, composed-object, literal-unit, stretch, or forbidden-helper violations. The four `car-scale` tests passed again after that change.

Eleven of the twenty-six lesson masters were still unaccepted at the last production checkpoint, so the full-course preview is not joined yet. No second render was admitted beside the running suite.
