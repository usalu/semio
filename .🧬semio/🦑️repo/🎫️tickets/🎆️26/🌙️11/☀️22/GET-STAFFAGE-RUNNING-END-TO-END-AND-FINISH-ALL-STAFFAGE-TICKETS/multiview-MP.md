FINAL

# Multiview Re-Proof (MP, 2026-11-26)

Each asset: scan (`🔎️scan.ts`), check CLI (`--asset`), matrix (`--jobs 1 --assets`) on the catalog file as hashed. `reported` counts group-drift findings (tier reported, not hard). `unavail` cells are declared unavailable representations, not failures. Logs: `🗑️generated/matrix/MP/logs/<id>.{scan,check,matrix}.log` (+ `.scan.json`), matrix records `🗑️generated/matrix/MP/<id>/`.

| asset | sha256 | scan hard (reported) | check CLI | matrix ok/unavail/fail | log path |
|---|---|---|---|---|---|
| `apple-tree` | `f20aff248bbe9ccce26005bb9f793ca0172fffb1631bfe02fdcf4ea3b77ecebd` | 0 (108) | exit 0, ✓1 ✗0 | 84 ok / 0 unavail / 0 fail of 252 | `logs/apple-tree.*`, `apple-tree/` |
| `bamboo-clump` | `2be5546d38ee12037e6a39725be99d3b5c9fd09dc1482a612d76f479f8822687` | 0 (28) | exit 0, ✓1 ✗0 | 0 ok / 0 unavail / 0 fail of 252 | `logs/bamboo-clump.*`, `bamboo-clump/` |
| `bookshelf` | `7fbeab5acbb2b49eee462f4fc9b91ecc0a8ad0e7679db49da515bf4659a9676e` | 0 (0) | exit 0, ✓1 ✗0 | 252 ok / 0 unavail / 0 fail of 252 | `logs/bookshelf.*`, `bookshelf/` |
| `charcoal-grill` | `867c63192229d4816b853f6561e342d15d7eb0fad5b699c5554ca0b0e0d22671` | 0 (7) | exit 0, ✓1 ✗0 | 14 ok / 0 unavail / 0 fail of 252 | `logs/charcoal-grill.*`, `charcoal-grill/` |
| `common-lilac` | `dfcc5eca9b4e76be231cc0c80e8855ee953cfa7b723d4d7c48ad298bc4990540` | 0 (0) | exit 0, ✓1 ✗0 | 252 ok / 0 unavail / 0 fail of 252 | `logs/common-lilac.*`, `common-lilac/` |
| `black-locust` | `5c381077d96c3b23f9e4cdd5550f24429019f6e154aa7cac7d9de6eb32668c02` | 0 (25) | exit 0, ✓1 ✗0 | 0 ok / 0 unavail / 0 fail of 252 | `logs/black-locust.*`, `black-locust/` |
| `cafe-chair` | `1867ea4264942e040f2cc8711b3236d5a65551dc26a36cf9ad77f06f5587e873` | 0 (2) | exit 0, ✓1 ✗0 | 154 ok / 0 unavail / 0 fail of 252 | `logs/cafe-chair.*`, `cafe-chair/` |
| `crab-apple` | `c4ba2dfe3f120c51be41efe369b35f36e4d3b91b5f21a154bc85ada6da7571d0` | 0 (17) | exit 0, ✓1 ✗0 | 105 ok / 0 unavail / 0 fail of 252 | `logs/crab-apple.*`, `crab-apple/` |
| `common-ash` | `fb0cf7198cb456b0d4e6849bfbef3faa0a7f16a410b01433323365bebe53c9e9` | 0 (1) | exit 0, ✓1 ✗0 | 224 ok / 0 unavail / 0 fail of 252 | `logs/common-ash.*`, `common-ash/` |
| `common-reed` | `8e5a85e572954eb522ee0d03beaa5e06825647050917cad5dc55811b52a80a69` | 0 (15) | exit 0, ✓1 ✗0 | 35 ok / 0 unavail / 0 fail of 252 | `logs/common-reed.*`, `common-reed/` |
| `curtain-wall-bay` | `4e5730191cbaaa316cafe11b9e5859ed56435a67ec910be2983e5102f7dc5ff9` | 0 (0) | exit 0, ✓1 ✗0 | 252 ok / 0 unavail / 0 fail of 252 | `logs/curtain-wall-bay.*`, `curtain-wall-bay/` |
| `decentralised-hrv-fan` | `11f8c03d753ed7b671330c3c23c6e54e0b2252e771b368403c5765cb9d1ba0c6` | 0 (12) | exit 0, ✓1 ✗0 | 42 ok / 21 unavail / 0 fail of 252 | `logs/decentralised-hrv-fan.*`, `decentralised-hrv-fan/` |
| `duct-fan` | `6beb2f49aba56811319b2f3ac007fe2340390f20b3a7c067bd37f9df97164895` | 0 (18) | exit 0, ✓1 ✗0 | 0 ok / 21 unavail / 0 fail of 252 | `logs/duct-fan.*`, `duct-fan/` |
| `dawn-redwood` | `018a8e5cb79d88f058c05ab5f413ff07cbd01e061f453b2da78a8dde17eb83b1` | 0 (0) | exit 0, ✓1 ✗0 | 252 ok / 0 unavail / 0 fail of 252 | `logs/dawn-redwood.*`, `dawn-redwood/` |
| `dining-table` | `544df18a72035882e3f5de74d5eced6fa180a0d9192d656cc0fe81cf3d8a97dd` | 0 (10) | exit 0, ✓1 ✗0 | 154 ok / 0 unavail / 0 fail of 252 | `logs/dining-table.*`, `dining-table/` |
| `english-oak` | `e2d3769429e0b1e99f584af0af3d4b3ffe5571489f6a2456910d1ec641a4c9fd` | 0 (8) | exit 0, ✓1 ✗0 | 182 ok / 0 unavail / 0 fail of 252 | `logs/english-oak.*`, `english-oak/` |
| `european-larch` | `7471368721f59293099c6b3bc951ed30c90feca7b2c1cb675e2cf5634f090a3c` | 0 (0) | exit 0, ✓1 ✗0 | 252 ok / 0 unavail / 0 fail of 252 | `logs/european-larch.*`, `european-larch/` |
| `european-beech` | `dd64681657b54a2b423772f47cc28d9c9fa775ee9d9dc33e8f05a82cca2e889b` | 0 (0) | exit 0, ✓1 ✗0 | 252 ok / 0 unavail / 0 fail of 252 | `logs/european-beech.*`, `european-beech/` |
| `external-venetian-blind` | `b6ffd717c49ee55b37eeceaa9e91972a45963f13094aac76b356f29dcbfbd703` | 0 (0) | exit 0, ✓1 ✗0 | 252 ok / 0 unavail / 0 fail of 252 | `logs/external-venetian-blind.*`, `external-venetian-blind/` |
| `gas-hob` | `b7543fcfe315fd609b6f9a4c0bc805ee7913dded3b54bcf0f70809e175477962` | 0 (6) | exit 0, ✓1 ✗0 | 147 ok / 21 unavail / 0 fail of 252 | `logs/gas-hob.*`, `gas-hob/` |
| `kitchen-extractor-hood` | `cd4dbb462b5b8e8b7849ab7815a12a9b308173e4d45ee5b205601674d8d6858e` | 0 (12) | exit 0, ✓1 ✗0 | 119 ok / 21 unavail / 0 fail of 252 | `logs/kitchen-extractor-hood.*`, `kitchen-extractor-hood/` |
| `park-bench` | `e92de8def49c252bc8836add99bf8839a51ef22eb0e9fae572cb577faf9d6922` | 0 (0) | exit 0, ✓1 ✗0 | 252 ok / 0 unavail / 0 fail of 252 | `logs/park-bench.*`, `park-bench/` |
| `reclaimed-steel-column` | `b633d899047ec1bc1b612cd531738886dea66ffe932194e1f9fb6a8547822e8b` | 0 (0) | exit 0, ✓1 ✗0 | 252 ok / 0 unavail / 0 fail of 252 | `logs/reclaimed-steel-column.*`, `reclaimed-steel-column/` |
| `sorption-wheel` | `5d1590791456d44bb32cac7899254b08078c0348486b85ebfff31c2ddb2530c8` | 0 (0) | exit 0, ✓1 ✗0 | 231 ok / 21 unavail / 0 fail of 252 | `logs/sorption-wheel.*`, `sorption-wheel/` |
| `stepladder` | `61b0cbbd4caafea156c9f0a49ea0d8a40fa89f4e47dc6238c20591db5c694041` | 0 (13) | exit 0, ✓1 ✗0 | 70 ok / 0 unavail / 0 fail of 252 | `logs/stepladder.*`, `stepladder/` |
| `used-bedside-table` | `d70315407cb1a7905dbc975121a93ab0547594ce231811b5ee459ee9e1ff16d8` | 0 (2) | exit 0, ✓1 ✗0 | 140 ok / 21 unavail / 0 fail of 231 | `logs/used-bedside-table.*`, `used-bedside-table/` |
| `used-cello` | `9265fb0e22a5aa494b4ac36bab71cc06c86d29523cbe74499af517b9ef95e54f` | 0 (18) | exit 0, ✓1 ✗0 | 35 ok / 0 unavail / 0 fail of 252 | `logs/used-cello.*`, `used-cello/` |
| `fig-tree` | `7ea1f72670ca1377ae62e0b710fb41d2e10052ce816de4fa888aba1b8319f451` | 0 (0) | exit 0, ✓1 ✗0 | 252 ok / 0 unavail / 0 fail of 252 | `logs/fig-tree.*`, `fig-tree/` |
| `hot-water-cylinder` | `14a95ca501ea4ad0e31069f7e7226f275253cf45962db71742dc6610bbcd7577` | 0 (24) | exit 0, ✓1 ✗0 | 70 ok / 0 unavail / 0 fail of 252 | `logs/hot-water-cylinder.*`, `hot-water-cylinder/` |
| `used-climbing-net-pyramid` | `6ab59b6dbf461448f26c39135c5e5865be10230d125424813cb72f012adf6245` | 0 (0) | exit 0, ✓1 ✗0 | 252 ok / 0 unavail / 0 fail of 252 | `logs/used-climbing-net-pyramid.*`, `used-climbing-net-pyramid/` |
| `used-crt-television` | `aefcfe24f41647a963944b75b0292ea4dabe708e4b5a1a6fe46eeb397c5bd94e` | 0 (0) | exit 0, ✓1 ✗0 | 252 ok / 0 unavail / 0 fail of 252 | `logs/used-crt-television.*`, `used-crt-television/` |
| `office-swivel-chair` | `93cba04613d4af1c27bba538732993cfb9251a4c8be3ba20e21feac0a5e86ef2` | 0 (39) | exit 0, ✓1 ✗0 | 21 ok / 0 unavail / 0 fail of 252 | `logs/office-swivel-chair.*`, `office-swivel-chair/` |
| `patio-heater` | `994dc29ac40a3b25adb82c65033cd7211fa65e171c8387fbd69ee6611c2323b1` | 0 (0) | exit 0, ✓1 ✗0 | 252 ok / 0 unavail / 0 fail of 252 | `logs/patio-heater.*`, `patio-heater/` |
| `used-djembe-and-cajon` | `4b39e48cbef93d9c58f8731e9b9e4f41804edc539f109790d7ad08be1d64084d` | 0 (6) | exit 0, ✓1 ✗0 | 189 ok / 21 unavail / 0 fail of 252 | `logs/used-djembe-and-cajon.*`, `used-djembe-and-cajon/` |
| `single-bed` | `7c305c009cfdb4e3bbbffa9e770a82c3422e19c544a56ddc3691449260daf934` | 0 (0) | exit 0, ✓1 ✗0 | 252 ok / 0 unavail / 0 fail of 252 | `logs/single-bed.*`, `single-bed/` |
| `used-double-swing` | `ce07d9fd833446d55e7de213acf2e18e279c5999e178203fbfd8f82c4593d419` | 0 (0) | exit 0, ✓1 ✗0 | 252 ok / 0 unavail / 0 fail of 252 | `logs/used-double-swing.*`, `used-double-swing/` |
| `used-electric-cooker` | `38260d7ea012426472b43c44ed5b808057fd9627ff8d6c1d2d6c446d70cb7883` | 0 (6) | exit 0, ✓1 ✗0 | 21 ok / 21 unavail / 0 fail of 252 | `logs/used-electric-cooker.*`, `used-electric-cooker/` |
| `used-electronic-drum-kit` | `cfbd84a8f890df1e7c1e2890c5eef97eb1fa583aaa9a015f7227c4094725afa4` | 0 (0) | exit 0, ✓1 ✗0 | 252 ok / 0 unavail / 0 fail of 252 | `logs/used-electronic-drum-kit.*`, `used-electronic-drum-kit/` |
| `used-extendable-dining-table` | `15df5603af05634f9a4ea9770ccdd7f29dff4c4962d771ea2d1ae3175150a09f` | 0 (0) | exit 0, ✓1 ✗0 | 252 ok / 0 unavail / 0 fail of 252 | `logs/used-extendable-dining-table.*`, `used-extendable-dining-table/` |
| `standing-adult` | `e3fae9c60d31881c8ee2a5d6798094a17eda672c9a01a1a4e3c65a3b84621bc6` | 0 (16) | exit 0, ✓1 ✗0 | 70 ok / 0 unavail / 0 fail of 252 | `logs/standing-adult.*`, `standing-adult/` |
| `used-fridge-freezer` | `8a9bdc92c1d3847ac24825f3362b2615b09a427665ad62a2bd9c5ab4a450c605` | 0 (12) | exit 0, ✓1 ✗0 | 140 ok / 0 unavail / 0 fail of 252 | `logs/used-fridge-freezer.*`, `used-fridge-freezer/` |
| `used-basketball-hoop` | `9a5525fff379585705769461738c86f1885a06996662cd0dadf0a98dde74f94e` | 0 (4) | exit 0, ✓1 ✗0 | 84 ok / 0 unavail / 0 fail of 252 | `logs/used-basketball-hoop.*`, `used-basketball-hoop/` |
| `used-kitchen-base-cabinet` | `baa5ebc91edefe8b4b56a712c0b10a64597b450d2540b1b5072c8e29d0c74c35` | 0 (12) | exit 0, ✓1 ✗0 | 140 ok / 0 unavail / 0 fail of 252 | `logs/used-kitchen-base-cabinet.*`, `used-kitchen-base-cabinet/` |
| `used-park-armchair` | `e1665198b9bee96260d027c5d86422cd86dd63a0ae2854aa0a70518bb9b265f8` | 0 (16) | exit 0, ✓1 ✗0 | 0 ok / 0 unavail / 0 fail of 252 | `logs/used-park-armchair.*`, `used-park-armchair/` |
| `used-bicycle-shelter` | `f03d873e01d3ad682ef6a61f3e43e50998c3fc945107b8e1c86bf14c1635e8bb` | 0 (0) | exit 0, ✓1 ✗0 | 231 ok / 0 unavail / 0 fail of 252 | `logs/used-bicycle-shelter.*`, `used-bicycle-shelter/` |
| `used-sandpit-digger` | `8844052f5efa3b47a02a3d611d7bc022066e82a306e3e2d95dfb67e4ac59b0f0` | 0 (0) | exit 0, ✓1 ✗0 | 252 ok / 0 unavail / 0 fail of 252 | `logs/used-sandpit-digger.*`, `used-sandpit-digger/` |
| `used-trench-convector` | `31aa0b338fb190cf0cfe8972da7b66a7fd505766bd1f6751608f671fe838b341` | 0 (0) | exit 0, ✓1 ✗0 | 252 ok / 0 unavail / 0 fail of 252 | `logs/used-trench-convector.*`, `used-trench-convector/` |
| `used-viola` | `63e19967b0669aa1a5f0d4d8ec9efeb9f565188c50dd3166f2b91b75811f1d7e` | 0 (1) | exit 0, ✓1 ✗0 | 119 ok / 21 unavail / 0 fail of 252 | `logs/used-viola.*`, `used-viola/` |
| `used-classical-guitar` | `b1f3dba7e70997a4ec5ae45510ec0227c518a12e78998a6e5921e374bc199933` | 0 (0) | exit 0, ✓1 ✗0 | 238 ok / 0 unavail / 0 fail of 252 | `logs/used-classical-guitar.*`, `used-classical-guitar/` |
| `used-water-play-table` | `097fb7e36cbfdb73b2360cbd221877e2d0aca83226b03c3442c64fadb4c1972f` | 0 (0) | exit 0, ✓1 ✗0 | 252 ok / 0 unavail / 0 fail of 252 | `logs/used-water-play-table.*`, `used-water-play-table/` |
| `used-concert-harp` | `4c9c661e23a09e5688594f2825ddb6ddce0361da5b6a90f758390a0ef2287397` | 0 (14) | exit 0, ✓1 ✗0 | 35 ok / 0 unavail / 0 fail of 252 | `logs/used-concert-harp.*`, `used-concert-harp/` |
| `used-wingback-armchair` | `1978db719462deb36ddf47584080264e170c90d967573c454c433e5697df3c84` | 0 (0) | exit 0, ✓1 ✗0 | 252 ok / 0 unavail / 0 fail of 252 | `logs/used-wingback-armchair.*`, `used-wingback-armchair/` |
| `used-dishwasher` | `8dc1d571868c2567414cebc214290c6b610b2e88875e1ada34c639701e8f04a7` | 0 (24) | exit 0, ✓1 ✗0 | 70 ok / 21 unavail / 0 fail of 231 | `logs/used-dishwasher.*`, `used-dishwasher/` |
| `used-double-bass` | `937f4761aab89e2fe758bb718313997ccabf5ec250e2fc9f1997c4471a9c7393` | 0 (0) | exit 0, ✓1 ✗0 | 252 ok / 0 unavail / 0 fail of 252 | `logs/used-double-bass.*`, `used-double-bass/` |
| `used-drum-kit` | `a21b17fba557997541675caf2f964c5bafadac89458d99fb2002e86416fa433e` | 0 (0) | exit 0, ✓1 ✗0 | 210 ok / 0 unavail / 0 fail of 252 | `logs/used-drum-kit.*`, `used-drum-kit/` |
| `used-electric-guitar-and-combo` | `40fea58df6c0be389e1f51e83fb55ff4f4b3de2bac1a4a407418709111aa5ac9` | 0 (0) | exit 0, ✓1 ✗0 | 252 ok / 0 unavail / 0 fail of 252 | `logs/used-electric-guitar-and-combo.*`, `used-electric-guitar-and-combo/` |
| `used-electronic-organ` | `a11716ff533b5def660f1a6c6126f95ac69d29821a1d3b54c3c3a1173988bd70` | 0 (0) | exit 0, ✓1 ✗0 | 210 ok / 21 unavail / 0 fail of 231 | `logs/used-electronic-organ.*`, `used-electronic-organ/` |
| `used-football-goal` | `12e78e93334f248d6e0ef961678b42b848b222d92c14db457f140d3d8a458970` | 0 (31) | exit 0, ✓1 ✗0 | 28 ok / 0 unavail / 0 fail of 252 | `logs/used-football-goal.*`, `used-football-goal/` |
| `used-high-chair` | `a2a03d9026c582823e4f04b8d4dac603ab1b10d2635fa84816694c64ab51a02d` | 0 (7) | exit 0, ✓1 ✗0 | 203 ok / 21 unavail / 0 fail of 252 | `logs/used-high-chair.*`, `used-high-chair/` |
| `used-microwave-oven` | `206ccd2064aaf48e1d28eb8fe60b279fa57269e0b50c0c3f123c262f67e76bae` | 0 (6) | exit 0, ✓1 ✗0 | 126 ok / 42 unavail / 0 fail of 210 | `logs/used-microwave-oven.*`, `used-microwave-oven/` |
| `used-recliner-chair` | `50acde1068b869f86dd33d330a29fa9d545b13d7563b97afd9175c96ea596b47` | 0 (0) | exit 0, ✓1 ✗0 | 252 ok / 0 unavail / 0 fail of 252 | `logs/used-recliner-chair.*`, `used-recliner-chair/` |
| `used-spring-rocker` | `876ff5f6f88784f8fabcaaaaed85003cd42d2d53d6e73930424f6f8adbc7bb1e` | 0 (0) | exit 0, ✓1 ✗0 | 210 ok / 21 unavail / 0 fail of 231 | `logs/used-spring-rocker.*`, `used-spring-rocker/` |
| `used-tumble-dryer` | `f7efe646577a13cdc7d513f70ff3d32dc53f1681768e094d9f5bb634b0c12ae7` | 0 (5) | exit 0, ✓1 ✗0 | 35 ok / 21 unavail / 0 fail of 252 | `logs/used-tumble-dryer.*`, `used-tumble-dryer/` |
| `used-washing-machine` | `1d5cdbe5088aa4c8b2518a264ba56abfd8fa0baabbac1849248baae8b534a99a` | 0 (0) | exit 0, ✓1 ✗0 | 210 ok / 0 unavail / 0 fail of 252 | `logs/used-washing-machine.*`, `used-washing-machine/` |
| `used-wc-pan-cistern` | `f7a5555f5d433c042408b86706f5f26e12b962cd47cc04337d3ad7572c8251f9` | 0 (20) | exit 0, ✓1 ✗0 | 140 ok / 21 unavail / 0 fail of 231 | `logs/used-wc-pan-cistern.*`, `used-wc-pan-cistern/` |
| `walking-commuter` | `e2e545c04c82670ac1de83f77c03c457abb5442ddf3a27ee91a8610e8ad7ba5f` | 0 (8) | exit 0, ✓1 ✗0 | 224 ok / 0 unavail / 0 fail of 252 | `logs/walking-commuter.*`, `walking-commuter/` |

Proven clean: 66 of 66.

## needs a fixer

none
