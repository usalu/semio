# Staffage export acceptance

At 2026-11-21 20:35 UTC, 138 of 152 current catalog documents have an independent twelve-scene GLB readback matching their exact source bytes. That covers 1,656 physical, thermal and structural scenes at 1:50, 1:100, 1:200 and 1:500.

30 of these documents also match the current validator dependency digest, `bbb57d68038e4dc35d3b398acb5247cc09c8363bc0f6503c497c30bf35c3639f`. Every reported GLB was measured within the 8 MiB publication budget.

The exporter produced each asset in its authored presentation pose. The independent loader checked its scene graph, geometry and material variants. Per-model reports also cover rig operation, heat transfer, physical outlines and paper suitability.

| Family | Asset | Source SHA-256 prefix | Readback | GLB bytes |
| --- | --- | --- | --- | ---: |
| vegetation | agave | `09d04e6dcd43226c` | Earlier validator | 5,286,748 |
| vegetation | apple-tree | `22da0dbd3167b69b` | Earlier validator | 3,419,784 |
| vegetation | bamboo-clump | `bfee357024857810` | Earlier validator | 6,797,744 |
| vegetation | date-palm | `ffda6f891e834a5e` | Earlier validator | 5,227,832 |
| vegetation | english-oak | `386417ae64321259` | Earlier validator | 5,011,192 |
| vegetation | hornbeam-hedge | `c4d9d09d6e873c5d` | Earlier validator | 5,922,092 |
| vegetation | hosta-clump | `404a79322758a747` | Earlier validator | 5,143,764 |
| vegetation | ivy-on-trellis | `1566b5e367b57f54` | Earlier validator | 6,147,536 |
| vegetation | japanese-maple | `ec5bc737a45d1f47` | Earlier validator | 2,700,008 |
| vegetation | lombardy-poplar | `055cbf34fb433a3c` | Earlier validator | 4,656,076 |
| vegetation | london-plane-pollarded | `8976385b4f2445df` | Earlier validator | 3,504,852 |
| vegetation | miscanthus-grass | `a21ab0f43079c442` | Earlier validator | 3,599,896 |
| vegetation | multi-stem-birch | `c14b7f870ee92f0d` | Earlier validator | 6,863,164 |
| vegetation | scots-pine | `6916c561558ae09b` | Earlier validator | 3,004,632 |
| vegetation | small-leaved-lime | `8c2fa916d4f073df` | Earlier validator | 3,735,944 |
| vegetation | weeping-willow | `8e2bc7f15a508910` | Earlier validator | 4,686,360 |
| machinery | concrete-mixer-truck | `cf8b029bbfd7d50c` | Current | 7,613,204 |
| machinery | diesel-generator | `7d5380a52c0f5739` | Pending | — |
| machinery | evaporative-humidifier | `14c5a3b53b7b764d` | Earlier validator | 942,996 |
| machinery | excavator | `7bd7ea353327a3d4` | Current | 5,159,532 |
| machinery | expansion-valve | `27330752a12403b0` | Earlier validator | 1,078,196 |
| machinery | finned-heat-exchanger | `555c7de5b65493e5` | Pending | — |
| machinery | forklift | `a3dba217f2bf2fe0` | Current | 3,525,596 |
| machinery | heat-pump-outdoor-unit | `61f3a11eeab250a9` | Pending | — |
| machinery | oil-pumpjack | `8a7f2f598ddcc926` | Current | 2,291,252 |
| machinery | plate-heat-exchanger | `d6c1149cb070acf8` | Earlier validator | 1,136,184 |
| machinery | scissor-lift | `551a3734f28b23e9` | Earlier validator | 2,475,076 |
| machinery | scroll-compressor | `8dc34a0cd4cf8b7d` | Pending | — |
| machinery | solar-thermal-collector | `b00511be504c32df` | Earlier validator | 804,444 |
| machinery | sorption-wheel | `630c785b9081f5a7` | Earlier validator | 1,320,616 |
| machinery | tower-crane | `ee9104e032eb4632` | Earlier validator | 8,191,280 |
| machinery | wheel-loader | `a6159e39915b2fb9` | Current | 3,379,716 |
| animals | dairy-cow | `447ff05f10ec0b4a` | Earlier validator | 7,009,612 |
| animals | domestic-cat | `69448bebe67800db` | Current | 3,540,908 |
| animals | goat | `ad18555ac0472c61` | Current | 4,749,452 |
| animals | hedgehog | `0dd623bd03e1edec` | Earlier validator | 4,171,116 |
| animals | herring-gull | `eab624a7a7b1d390` | Earlier validator | 4,783,956 |
| animals | horse | `2688225129140e79` | Earlier validator | 6,704,144 |
| animals | labrador-dog | `7fe1518b5d723750` | Earlier validator | 4,811,992 |
| animals | mallard-duck | `dbcb5d687f23ca6f` | Current | 2,397,864 |
| animals | pig | `a332d7f66f6ef313` | Current | 3,709,744 |
| animals | pigeon | `6f630afcea1f1dc3` | Earlier validator | 2,081,468 |
| animals | red-fox | `5bc3cf66c5515270` | Current | 3,419,236 |
| animals | red-squirrel | `5a60ab9dc872d682` | Earlier validator | 4,211,252 |
| animals | roe-deer | `0396319eb4d4ed77` | Current | 3,928,508 |
| animals | sheep | `13f3d6ecb1eff552` | Earlier validator | 3,453,300 |
| humans | child-on-kick-scooter | `0f25a5afc7da01a9` | Pending | — |
| humans | courier-with-hand-truck | `d4ee2ab94e08c6ac` | Current | 4,737,188 |
| humans | cyclist | `59656079ffd00b66` | Earlier validator | 5,484,956 |
| humans | elderly-with-cane | `f778bc6c3f9b5698` | Earlier validator | 6,455,896 |
| humans | jogger | `ebb573cdf6466f0a` | Current | 4,592,216 |
| humans | kneeling-gardener | `d6c4d1236ca3532f` | Current | 5,390,204 |
| humans | leaning-person | `2cd99a547173aa3e` | Current | 4,594,616 |
| humans | parent-carrying-toddler | `f2dc112ecd2c2d0a` | Earlier validator | 8,008,364 |
| humans | parent-with-stroller | `20bc27b692bbaf84` | Earlier validator | 7,149,800 |
| humans | person-cooking | `f0b1cc9f29841c74` | Current | 4,835,076 |
| humans | person-on-crutches | `25dd500ddb5dcab5` | Earlier validator | 7,296,768 |
| humans | person-with-rollator | `714c7b40c0330048` | Earlier validator | 7,743,152 |
| humans | person-with-umbrella | `082d65807152cc34` | Earlier validator | 5,957,112 |
| humans | person-with-white-cane | `1e32be0cf96b9723` | Pending | — |
| humans | pregnant-person | `c0bafbfcbe7cfbb1` | Earlier validator | 4,680,684 |
| humans | runner-with-blade-prosthesis | `79cca1d56a8312a5` | Current | 3,921,948 |
| humans | running-child | `a3d53630b01949b7` | Pending | — |
| humans | seated-guitarist | `77afbc683ca2d4ba` | Current | 4,309,636 |
| humans | seated-office-worker | `e8d6b135349f7a95` | Earlier validator | 5,409,104 |
| humans | seated-reader | `92547aecd0033827` | Current | 3,966,448 |
| humans | skateboarder | `9e30636b16b73e84` | Current | 4,231,024 |
| humans | sleeping-person | `d83c849d3ac6a66d` | Pending | — |
| humans | standing-adult | `8d6b852ff6aa2f8f` | Earlier validator | 5,872,764 |
| humans | standing-phone-user | `f1b13f7f5599def7` | Earlier validator | 6,095,816 |
| humans | tour-guide | `85ae55841b9d2b7e` | Current | 5,004,984 |
| humans | walking-commuter | `546a8320e9a5f78d` | Earlier validator | 6,545,060 |
| humans | wheelchair-user | `d97ce0fb43f1a1f7` | Earlier validator | 5,626,288 |
| humans | worker-carrying-box | `547a7895f3f97be1` | Earlier validator | 7,132,644 |
| appliances | ceiling-diffuser | `5641270bbbca5183` | Earlier validator | 696,788 |
| appliances | circulation-pump | `7a93dd4fd8fd5b78` | Earlier validator | 1,893,788 |
| appliances | decentralised-hrv-fan | `a17fb615698ee618` | Earlier validator | 1,560,936 |
| appliances | desktop-monitor | `a2aee05ac9910b12` | Earlier validator | 685,320 |
| appliances | duct-fan | `11237b05486324ce` | Earlier validator | 1,226,980 |
| appliances | electric-fan-heater | `bc0ee4103cc9bd16` | Earlier validator | 929,604 |
| appliances | electric-kettle | `6dc119550e0e16b6` | Earlier validator | 1,378,020 |
| appliances | electricity-meter | `dc2bc5885104188a` | Earlier validator | 750,232 |
| appliances | gas-boiler | `8d08cf336d44ab5a` | Earlier validator | 1,313,932 |
| appliances | gas-hob | `71144d9be90c1ef9` | Earlier validator | 1,278,112 |
| appliances | gas-meter | `42e0a6fd782ecf5f` | Earlier validator | 1,070,332 |
| appliances | hot-water-cylinder | `b609f96ac449236b` | Earlier validator | 2,605,028 |
| appliances | kitchen-extractor-hood | `dd6b6dbd06b21190` | Earlier validator | 1,225,036 |
| appliances | laptop | `9feb534af108f046` | Earlier validator | 920,048 |
| appliances | mvhr-unit | `765b627c8467984b` | Earlier validator | 1,301,320 |
| appliances | pendant-lamp | `9871974113e3e6cc` | Earlier validator | 1,097,788 |
| appliances | room-thermostat | `470f6f918458a45a` | Earlier validator | 480,908 |
| appliances | server-rack | `89d8de0849af9bd0` | Current | 1,839,872 |
| appliances | shower-head | `7b48e9e0fdab1374` | Earlier validator | 1,073,640 |
| appliances | split-ac-indoor-unit | `db3fdfea58f5fe1a` | Earlier validator | 723,288 |
| appliances | thermostatic-radiator-valve | `964c42b769f1860c` | Earlier validator | 1,087,468 |
| appliances | toaster | `aa10e3d982907c75` | Earlier validator | 786,164 |
| appliances | tower-server | `e67368f4b3487c26` | Earlier validator | 1,616,772 |
| appliances | vacuum-cleaner | `28dd9d675ed27f42` | Earlier validator | 2,365,104 |
| vehicles | cargo-bike | `90a085a747a73ebe` | Current | 3,655,808 |
| vehicles | city-bicycle | `92fda43223ecc133` | Current | 3,058,224 |
| vehicles | city-bus | `869b31e8fd31b410` | Current | 6,705,868 |
| vehicles | city-car | `fc967fdf7bcef9c2` | Earlier validator | 8,378,128 |
| vehicles | delivery-van | `e3a1daa411f83e3d` | Earlier validator | 8,010,960 |
| vehicles | electric-kick-scooter | `057c3f0d8113d288` | Pending | — |
| vehicles | motor-scooter | `77dc2e0852376fb3` | Pending | — |
| vehicles | tram | `f9218a9f5694bc2f` | Current | 5,167,652 |
| vehicles | vintage-car-1929 | `0d14a9330b0353db` | Current | 6,259,192 |
| furniture | bar-stool | `4959723c58e242bb` | Earlier validator | 1,259,664 |
| furniture | bookshelf | `cd3cdd4d958016ab` | Earlier validator | 876,272 |
| furniture | cafe-chair | `aaf340858e5c4ad7` | Earlier validator | 966,136 |
| furniture | dining-table | `a2fd99e3124a01c7` | Earlier validator | 1,134,880 |
| furniture | folding-deck-chair | `97e09e0fcb1947ec` | Earlier validator | 1,227,816 |
| furniture | height-adjustable-desk | `c8f4d5030e89693e` | Earlier validator | 1,237,296 |
| furniture | office-swivel-chair | `c2acca38be648d79` | Current | 2,348,300 |
| furniture | operating-table | `bfcfaa8be2f4f794` | Earlier validator | 2,441,288 |
| furniture | park-bench | `0f7555a3acf0ae7f` | Earlier validator | 897,464 |
| furniture | picnic-table | `d0ada155e3720ac8` | Earlier validator | 1,049,948 |
| furniture | single-bed | `9e51236df2755448` | Earlier validator | 1,993,996 |
| furniture | three-seat-sofa | `542e3357b377bc47` | Earlier validator | 1,260,924 |
| components | cantilever-balcony | `db754982faa9290d` | Earlier validator | 1,023,184 |
| components | curtain-wall-bay | `341b36ada1392488` | Earlier validator | 833,988 |
| components | etics-brick-wall-12 | `bd5123f946a20a72` | Pending | — |
| components | etics-brick-wall-30 | `ab9131b3af0c87be` | Pending | — |
| components | external-venetian-blind | `4c9da985f921f3c9` | Pending | — |
| components | glass-balustrade | `7e2bf343d77915c7` | Earlier validator | 839,660 |
| components | glulam-beam | `1a16f69e5ed56978` | Earlier validator | 628,552 |
| components | masonry-wall-with-lintel | `de8821aaa0a0d86f` | Earlier validator | 1,359,976 |
| components | panel-radiator | `ce5fa751dce33264` | Pending | — |
| components | sectional-garage-door | `2d1cf739511c0fae` | Earlier validator | 3,946,868 |
| components | sliding-door | `f97caf963a10c338` | Earlier validator | 858,980 |
| components | solid-brick-wall-1960 | `ca4d8f535ec2be4d` | Earlier validator | 523,616 |
| components | spiral-stair | `a523e221564f23ff` | Current | 1,864,656 |
| components | stair-flight | `f8a7b4783b509797` | Earlier validator | 703,196 |
| components | steel-column | `6d464c13d2ab719f` | Earlier validator | 686,336 |
| components | tied-canopy | `c555e1f5365bd9c3` | Earlier validator | 1,402,348 |
| components | tilt-window | `7d14394579d98f49` | Earlier validator | 590,172 |
| components | wheelchair-ramp | `89dde8aca7c88994` | Earlier validator | 1,268,680 |
| objects | bicycle-stand | `dc2d9509ab2a859f` | Earlier validator | 700,380 |
| objects | chalkboard | `12bda0de5aa079df` | Earlier validator | 766,860 |
| objects | charcoal-grill | `86cd6e5208a34587` | Earlier validator | 3,833,896 |
| objects | cooking-pot | `4548a6dc60ec5a72` | Earlier validator | 1,417,924 |
| objects | drinking-fountain | `3265f187c0da1268` | Earlier validator | 1,930,652 |
| objects | market-parasol | `a734c7c861a2d377` | Current | 2,075,236 |
| objects | patio-heater | `4f4f75a3cc1efd90` | Earlier validator | 2,830,308 |
| objects | playground-swing | `1c20eb9a3440b9ae` | Earlier validator | 2,777,768 |
| objects | retractable-bollard | `ce625bb0b8e065c4` | Earlier validator | 1,323,872 |
| objects | shopping-trolley | `9669dc888d54546d` | Earlier validator | 2,856,124 |
| objects | stepladder | `f3320c4bb47e71d2` | Earlier validator | 1,123,696 |
| objects | storage-crate-stack | `4bfe28235ab7a318` | Earlier validator | 1,030,340 |
| objects | street-lamp | `50897e941e38ca88` | Earlier validator | 1,287,080 |
| objects | tealight | `e20b2bd2698c68d8` | Earlier validator | 758,584 |
| objects | waste-bin | `16549680e29c3f4d` | Earlier validator | 1,110,480 |
