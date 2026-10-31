# Coverage matrix

Every asset × preset scale × view × (named pose | clip at mid-time). 10184 rows (full detail in the CSV `🗑️generated/core/coverage-matrix.csv`), 10159 without failures. Unavailable scales are decisions of the asset (no export). "svg/pdf" counts are rows (7 views × queries) where both exports succeeded / rows. Export formats: svg, pdf from `depict`; dxf from `modelScene`. Transport overlay is drawn once per asset on the default pose.

| asset | 1:500 | 1:200 | 1:100 | 1:50 | thermal regions | emitters | transport overlay | dxf failures | failing rows |
|---|---|---|---|---|---|---|---|---|---|
| cyclist | unsuitable 49/49 | unsuitable 49/49 | unsuitable 49/49 | unsuitable 49/49 | yes | 3 | ok | 0 | 0 |
| elderly-with-cane | unsuitable 49/49 | unsuitable 49/49 | unsuitable 49/49 | unsuitable 49/49 | yes | 3 | ok | 0 | 0 |
| jogger | unsuitable 56/56 | unsuitable 56/56 | unsuitable 56/56 | unsuitable 56/56 | yes | 3 | ok | 0 | 0 |
| leaning-person | unsuitable 49/49 | unsuitable 49/49 | unsuitable 49/49 | unsuitable 49/49 | yes | 3 | ok | 0 | 0 |
| parent-with-stroller | unsuitable 49/49 | unsuitable 49/49 | unsuitable 49/49 | unsuitable 49/49 | yes | 4 | ok | 0 | 0 |
| person-with-rollator | unavailable | unavailable | unavailable | unsuitable 42/42 | yes | 3 | ok | 0 | 0 |
| person-with-umbrella | unsuitable 35/35 | unsuitable 35/35 | unsuitable 35/35 | unsuitable 35/35 | yes | 3 | ok | 0 | 0 |
| running-child | unsuitable 56/56 | unsuitable 56/56 | unsuitable 56/56 | unsuitable 56/56 | yes | 3 | ok | 0 | 0 |
| seated-reader | unsuitable 49/49 | unsuitable 49/49 | unsuitable 49/49 | unsuitable 49/49 | yes | 3 | ok | 0 | 0 |
| skateboarder | unsuitable 49/49 | unsuitable 49/49 | unsuitable 49/49 | unsuitable 49/49 | yes | 3 | ok | 0 | 0 |
| standing-phone-user | unsuitable 35/35 | unsuitable 35/35 | unsuitable 35/35 | unsuitable 35/35 | yes | 3 | ok | 0 | 0 |
| tour-guide | unsuitable 42/42 | unsuitable 42/42 | unsuitable 42/42 | unsuitable 42/42 | yes | 3 | ok | 0 | 0 |
| walking-commuter | unsuitable 49/49 | unsuitable 49/49 | unsuitable 49/49 | unsuitable 49/49 | yes | 3 | ok | 0 | 0 |
| wheelchair-user | unsuitable 49/49 | unsuitable 49/49 | unsuitable 49/49 | unsuitable 49/49 | yes | 3 | ok | 0 | 0 |
| worker-carrying-box | unsuitable 56/56 | unsuitable 56/56 | unsuitable 56/56 | unsuitable 56/56 | yes | 3 | ok | 0 | 0 |
| dairy-cow | unsuitable 56/56 | unsuitable 56/56 | unsuitable 56/56 | unsuitable 56/56 | yes | 2 | ok | 0 | 0 |
| domestic-cat | unavailable | unsuitable 56/56 | unsuitable 56/56 | unsuitable 56/56 | yes | 3 | ok | 0 | 0 |
| horse | unsuitable 35/35 | unsuitable 35/35 | unsuitable 35/35 | unsuitable 35/35 | yes | 2 | ok | 0 | 0 |
| labrador-dog | unsuitable 49/49 | unsuitable 49/49 | unsuitable 49/49 | unsuitable 49/49 | yes | 3 | ok | 0 | 0 |
| mallard-duck | unavailable | unsuitable 49/49 | unsuitable 49/49 | unsuitable 49/49 | yes | 2 | ok | 0 | 0 |
| pigeon | unavailable | unsuitable 70/70 | unsuitable 70/70 | unsuitable 70/70 | yes | 2 | ok | 0 | 0 |
| roe-deer | unsuitable 56/56 | unsuitable 56/56 | unsuitable 56/56 | unsuitable 56/56 | yes | 2 | ok | 0 | 0 |
| date-palm | unsuitable 42/42 | unsuitable 42/42 | unsuitable 42/42 | unsuitable 42/42 | yes | 1 | ok | 0 | 0 |
| english-oak | unsuitable 49/49 | unsuitable 49/49 | unsuitable 49/49 | unsuitable 49/49 | yes | 2 | ok | 0 | 0 |
| hornbeam-hedge | authored 49/49 | unsuitable 49/49 | unsuitable 49/49 | unsuitable 49/49 | yes | 2 | ok | 0 | 0 |
| lombardy-poplar | authored 42/42 | unsuitable 42/42 | unsuitable 42/42 | unsuitable 42/42 | yes | 1 | ok | 0 | 0 |
| london-plane-pollarded | authored 56/56 | unsuitable 56/56 | unsuitable 56/56 | unsuitable 56/56 | yes | 2 | ok | 0 | 0 |
| miscanthus-grass | authored 49/49 | unsuitable 49/49 | unsuitable 49/49 | unsuitable 49/49 | yes | 1 | ok | 0 | 0 |
| multi-stem-birch | unsuitable 49/49 | unsuitable 49/49 | unsuitable 49/49 | unsuitable 49/49 | yes | 1 | ok | 0 | 0 |
| scots-pine | authored 49/49 | unsuitable 49/49 | unsuitable 49/49 | unsuitable 49/49 | yes | 2 | ok | 0 | 0 |
| weeping-willow | unsuitable 42/42 | unsuitable 42/42 | unsuitable 42/42 | unsuitable 42/42 | yes | 1 | ok | 0 | 0 |
| cafe-chair | unsuitable 42/42 | unsuitable 42/42 | unsuitable 42/42 | unsuitable 42/42 | yes | 0 | n/a | 0 | 0 |
| height-adjustable-desk | unsuitable 35/35 | unsuitable 35/35 | unsuitable 35/35 | unsuitable 35/35 | yes | 0 | n/a | 0 | 0 |
| office-swivel-chair | unsuitable 35/35 | unsuitable 35/35 | unsuitable 35/35 | unsuitable 35/35 | yes | 0 | n/a | 0 | 0 |
| park-bench | authored 35/35 | unsuitable 35/35 | unsuitable 35/35 | unsuitable 35/35 | yes | 0 | n/a | 0 | 0 |
| three-seat-sofa | authored 56/56 | authored 56/56 | authored 56/56 | authored 56/56 | yes | 0 | n/a | 0 | 0 |
| bicycle-stand | authored 42/42 | authored 42/42 | authored 42/42 | authored 42/42 | yes | 0 | n/a | 0 | 0 |
| market-parasol | unsuitable 35/35 | unsuitable 35/35 | unsuitable 35/35 | unsuitable 35/35 | yes | 0 | n/a | 0 | 0 |
| playground-swing | unsuitable 35/35 | unsuitable 35/35 | unsuitable 35/35 | unsuitable 35/35 | yes | 0 | n/a | 0 | 0 |
| stepladder | unsuitable 35/35 | unsuitable 35/35 | unsuitable 35/35 | unsuitable 35/35 | yes | 0 | n/a | 0 | 0 |
| street-lamp | unsuitable 49/49 | unsuitable 49/49 | unsuitable 49/49 | unsuitable 49/49 | yes | 1 | ok | 0 | 0 |
| waste-bin | unsuitable 35/35 | unsuitable 35/35 | unsuitable 35/35 | unsuitable 35/35 | yes | 0 | n/a | 0 | 0 |
| cargo-bike | unsuitable 35/35 | unsuitable 35/35 | unsuitable 35/35 | unsuitable 35/35 | yes | 0 | n/a | 0 | 0 |
| city-bicycle | unsuitable 42/42 | unsuitable 42/42 | unsuitable 42/42 | unsuitable 42/42 | yes | 0 | n/a | 0 | 0 |
| city-bus | unsuitable 49/49 | unsuitable 49/49 | unsuitable 49/49 | unsuitable 49/49 | yes | 1 | ok | 0 | 0 |
| city-car | authored 49/49 | unsuitable 49/49 | unsuitable 49/49 | unsuitable 49/49 | yes | 1 | ok | 0 | 0 |
| concrete-mixer-truck | unsuitable 42/42 | unsuitable 42/42 | unsuitable 42/42 | unsuitable 42/42 | yes | 2 | ok | 0 | 0 |
| excavator | unsuitable 42/42 | unsuitable 42/42 | unsuitable 42/42 | unsuitable 42/42 | yes | 1 | ok | 0 | 0 |
| forklift | unsuitable 34/35 | unsuitable 34/35 | unsuitable 34/35 | unsuitable 34/35 | yes | 1 | ok | 0 | 4 |
| tower-crane | unsuitable 42/42 | unsuitable 42/42 | unsuitable 42/42 | authored 42/42 | yes | 1 | ok | 0 | 0 |
| cantilever-balcony | authored 42/42 | authored 42/42 | authored 42/42 | unsuitable 42/42 | yes | 0 | n/a | 0 | 0 |
| glass-balustrade | unavailable | unsuitable 42/42 | unsuitable 42/42 | unsuitable 42/42 | yes | 0 | n/a | 0 | 0 |
| glulam-beam | authored 35/35 | authored 35/35 | unsuitable 35/35 | authored 35/35 | yes | 0 | n/a | 0 | 0 |
| panel-radiator | unavailable | unsuitable 42/49 | unsuitable 42/49 | unsuitable 42/49 | yes | 2 | ok | 0 | 21 |
| sliding-door | unsuitable 42/42 | unsuitable 42/42 | authored 42/42 | authored 42/42 | yes | 0 | n/a | 0 | 0 |
| stair-flight | unsuitable 35/35 | unsuitable 35/35 | unsuitable 35/35 | unsuitable 35/35 | yes | 0 | n/a | 0 | 0 |
| steel-column | authored 35/35 | unsuitable 35/35 | unsuitable 35/35 | unsuitable 35/35 | yes | 0 | n/a | 0 | 0 |
| tied-canopy | authored 35/35 | authored 35/35 | unsuitable 35/35 | unsuitable 35/35 | yes | 0 | n/a | 0 | 0 |
| tilt-window | unavailable | unsuitable 42/42 | unsuitable 42/42 | unsuitable 42/42 | yes | 0 | n/a | 0 | 0 |

## First failures per asset

- forklift 1:500 clip:drive-with-load@mid thermal: svg=ok pdf=fail:PDF colour must be #rgb or #rrggbb  got  #NaNNaNNaN  dxf=ok thermal=ok overlay=-
- forklift 1:200 clip:drive-with-load@mid thermal: svg=ok pdf=fail:PDF colour must be #rgb or #rrggbb  got  #NaNNaNNaN  dxf=ok thermal=ok overlay=-
- forklift 1:100 clip:drive-with-load@mid thermal: svg=ok pdf=fail:PDF colour must be #rgb or #rrggbb  got  #NaNNaNNaN  dxf=ok thermal=ok overlay=-
- panel-radiator 1:200 pose:off thermal: svg=fail:null is not an object (evaluating 'x.toFixed') pdf=fail:null is not an object (evaluating 'x.toFixed') dxf=ok thermal=ok overlay=-
- panel-radiator 1:100 pose:off thermal: svg=fail:null is not an object (evaluating 'x.toFixed') pdf=fail:null is not an object (evaluating 'x.toFixed') dxf=ok thermal=ok overlay=-
- panel-radiator 1:50 pose:off thermal: svg=fail:null is not an object (evaluating 'x.toFixed') pdf=fail:null is not an object (evaluating 'x.toFixed') dxf=ok thermal=ok overlay=-
