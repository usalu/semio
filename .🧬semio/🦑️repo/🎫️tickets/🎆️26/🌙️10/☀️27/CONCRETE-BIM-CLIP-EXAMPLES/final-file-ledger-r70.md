# Final File Ledger R70

This ledger uses only existing recorded per-file manifests; no Git command, working-tree scan, mtime inference or source mutation was performed. It records the integration candidate path union for eventual ticket close, not a proven list of Git-modified files. A recorded dependency can be unchanged from the repository baseline. No baseline-vs-working-tree manifest exists here, so attributing all dependencies as changed would be unsupported.

The union contains 176 clip-relative paths:29production Python dependencies,140core oracle paths and7additional feature-capture paths. Source/schema/native fixtures are included in the additional paths. Hashes below are the final feature-capture values where recorded, otherwise the completed core after-manifest values. Paths are relative to C:/git/semio/🎓️teaching/🏛️architecture/🏗️bim/🎬️clip.

Inputs: generated/clips-baseline-r33-before-manifest.json and after-manifest.json; generated/clips-cohort-r57-code-manifests.json; generated/features-r68-inputs.json. Final source capture is d2f3c0d1bb97c93836fb61b53adfe94756c59c31217585899decf09cda4f59f2. The ongoing parent/R40 feature test-only repair may supersede individual oracle hashes after this capture and must be appended by its owner. This ledger does not claim that ongoing feature execution completed.

## Captured Candidate Union

| Path | Recorded category | Captured SHA256 |
|---|---|---|
| 🎨️visuals/🏛️savoye/✂️section/🐍️.py | production | c90bbc6abb2a510a5871bbae7d7b85add9d51bb266d0a83447ac2b65d41cedcc |
| 🎨️visuals/🏛️savoye/🐍️.py | production | 896b299a6a9b31ad423e6f5ed1e01b71403516a3bf478ab277ec0477e3bb534a |
| 🎨️visuals/🏛️savoye/📋️plan/🐍️.py | production | b52d281d243fe3c28e72245e42461369bf1f407741999e53f10638485a3ecff5 |
| 🎨️visuals/🏛️savoye/📏️sheet/🐍️.py | production | 9f5f16481ee4c6dbc0bd9dd26123e347617da31566c32104007e5ad6cc52f26b |
| 🎨️visuals/🏛️savoye/🔩️detail/🐍️.py | production | 9ef27c3d212c5720de1b440d660b8ab013f6c568c74faa89f0d1d1206dca48b5 |
| 🎨️visuals/🏛️savoye/🧊️axon/🐍️.py | production | e583021de039882e4bdb5fb19da5470604491fec93b73a3e232ecdb7705c547b |
| 🎨️visuals/🏛️savoye/🧊️axon/📏️geometry.json | additional feature input | 32820c35faa63a0de6ef9576d64f531ba55a347556d26c4405b3c34831b2110d |
| 🎨️visuals/🏛️savoye/🧊️axon/📏️geometry.py | production | d13396b4fc18db17761bf90eaaf6cf8422dd67e0a9f0d5b0860f64d687d4c6c2 |
| 🎨️visuals/🏛️savoye/🧊️axon/🧩️primitives.py | production | 1677b211a74e78907a5a960705e284771d78785f4d9625f892372cc6fa9acb77 |
| 🎨️visuals/🏛️savoye/🧊️axon/🧪️cases.json | oracle | ebbafad5c7a4594a040e318248e0397bfedd8cfcc2d92a2342fefa210092ca9f |
| 🎨️visuals/🏛️savoye/🧊️axon/🧪️tests.py | oracle | 97f3f770b4463dc0a6e773628dcea0dc48574990d97acc88b0e3ef9125150075 |
| 🎨️visuals/🏛️savoye/🧪️ortho-remodel/cases.json | oracle | 6d8dd11368e9657228297f5f840c1a8355061c7476616a70fc900dacfb575944 |
| 🎨️visuals/🏛️savoye/🧪️ortho-remodel/frustums.json | oracle | 3881406eb89f48c06f08759010bcab3cbbe9e31a62106296e972f966d3c8cea9 |
| 🎨️visuals/🏛️savoye/🧪️ortho-remodel/profiles.json | oracle | 65ac4e8596769247d1c1d096f7f6196759d37b2e7e7e9c6bf29baf177e8dace9 |
| 🎨️visuals/🏛️savoye/🧪️ortho-remodel/screens.json | oracle | d965a1bd81d0094cb7def4b2110a0b208bd06ebd7a5c3eeb26fb547921f6497d |
| 🎨️visuals/🏛️savoye/🧪️ortho-remodel/services.json | oracle | b34c9e933f0da153312fd5fad55d6e8f34312e1bba3d310471eb3e7fb670319e |
| 🎨️visuals/🏛️savoye/🧪️ortho-remodel/slab-regions.json | oracle | 41f0112610ee26a214cd1f74235b3b34fbd7b231898d3996ee8ba061fa9ec1e8 |
| 🎨️visuals/🏛️savoye/🧪️ortho-remodel/stairs.json | oracle | caa67a45f0be7dea5734084badf3eeb6670c593e1620030471004c9b1319a79a |
| 🎨️visuals/🏛️savoye/🧪️ortho-remodel/test_cells.py | oracle | a75141ffe77dca21a1786624d8c386f185e6ca14592ff76189d3efd69cfc8ea4 |
| 🎨️visuals/🏛️savoye/🧪️ortho-remodel/test_defaults.py | oracle | 6eb9b5255cac56dd35ced88c0ff6cb1680ee26923fd3e49fc2de8c32f313ab1d |
| 🎨️visuals/🏛️savoye/🧪️ortho-remodel/test_frustums.py | oracle | d4447a916983722b6d216540ea10a80556bf0c16fa75d062a863667fad893610 |
| 🎨️visuals/🏛️savoye/🧪️ortho-remodel/test_profile_consumer.py | oracle | df6aa372e64e0991689c76e4659403f5e60cc351ad572583bdb954c80e047fc2 |
| 🎨️visuals/🏛️savoye/🧪️ortho-remodel/test_profiles.py | oracle | ee59fcdf60368ca0e6f18ec1a111716a74429bf6f1e93c18f3439ba6ac550d19 |
| 🎨️visuals/🏛️savoye/🧪️ortho-remodel/test_screens.py | oracle | c83543334e058f5f17c2687dabb2f40ef98b79d0ec43eb267b8e8fd069c35ba2 |
| 🎨️visuals/🏛️savoye/🧪️ortho-remodel/test_services.py | oracle | eca7482d15e475dc04d68eff348492d9339ae7caec83cf46f76edccf6fd9fb81 |
| 🎨️visuals/🏛️savoye/🧪️ortho-remodel/test_services_native.py | oracle | c70b7f3636994817fbb861b9eff8a3dd0eb89889237fc6fb534fcbc2826106f2 |
| 🎨️visuals/🏛️savoye/🧪️ortho-remodel/test_slab_regions.py | oracle | b4efad827711ae431306537fae9455e186ce40981e16c6d287bc327d0d8d513d |
| 🎨️visuals/🏛️savoye/🧪️ortho-remodel/test_stairs.py | oracle | 3640087c2930b4ea7aa0f037535ca9db035610d85f0617919162d019193b3915 |
| 🎨️visuals/🏛️savoye/🧱️core/🐍️.py | production | e0ec2dd1c4d67f5bca65c55bff107e2cebd56c1c05e04ebc025ab7448155efdc |
| 🎨️visuals/🏛️savoye/🧱️core/📐️geometry.py | production | 2fe107710f335d600ef81c18b39ed1b2a86758b86ac531d95eb15bedc11d0eb8 |
| 🎨️visuals/🏛️savoye/🧱️core/🧪️surfaces.json | oracle | 98099dfc0dc741b60705ec55a6a6defd2e32a77b3bfb48634dd357a22d27f928 |
| 🎨️visuals/🏛️savoye/🧱️core/🧪️surfaces.py | oracle | feb6dae484df0a6acb69881f1ad09db61733bbf2583c2752badc49353efde870 |
| 🎨️visuals/🐍️.py | production | ac4cd90085b0ba992c2a8e2b4779b852a761b2faf9ebaa61de18cffc2cb79641 |
| 🎨️visuals/📐️drafting/🐍️.py | production | 6dc660ac043c437f218be39c2287cf0551af50eb2fa5f3174485efc463b56088 |
| 🎨️visuals/📐️drafting/📏️geometry.py | production | 9d506aabadf86cfe9efa93b0f53ec0c50f4a27b8f1b476c515588bb530e74c18 |
| 🎨️visuals/🧮️analysis/🐍️.py | production | 1d0f736afc78a744b4a0262e144e97eadb88b4d308d381269abc71be1edc1de9 |
| 🎨️visuals/🧮️analysis/🧪️profiles/🐍️.py | oracle | 3d5e0105f4df807dc4f4091e442c65c91c5dde923826079e6c760504d9ef254d |
| 🎨️visuals/🧮️analysis/🧪️profiles/🔣️.json | oracle | a2d10505711f6a80f5eb6c304b73151c22b23faae521ddcef9b95eb7875fc6ae |
| 🎨️visuals/🧾️panels/🐍️.py | production | 813410b90a8f6c0ea66e59839b961a5e78ea88b548d0362748382f9d1835a76f |
| 🐍️.py | production | c5156dc9e106dc7a867d0ec5270c509a3e33254198831eaa8301407e1d7fe15d |
| 👁️representations/🐍️.py | production | 615c17f54501553c53ca3e0ba5bf55bdccebe26c85b1bbc2d2ef77990632e1ad |
| 📊️information/🐍️.py | production | 305736dc0280af9dfcf855a9d93754451ebcfe796e478ab943b3a461b202b52f |
| 🔌️exchange/🐍️.py | production | 893be454ec5ebea12f0c66b225a350ad10e6b946ad00219eaea38b94c434be79 |
| 🔌️exchange/🧪️cases/🔣️slab.json | oracle | ae48bfa80649953fb320c4803333ca1f12f23949c945b9866c9c133626f51bcd |
| 🔌️exchange/🧪️tests/🐍️slab.py | oracle | 96dd5f28507f31c31c016d3ce30aec21a9fe72bd4098a116e1ac10697136ce42 |
| 🔗️consistency/🐍️.py | production | 42ca62c35044e4cd929a0d54c880c7a1a8c2f584923f3ffe225ec37e6625eafe |
| 🔗️consistency/🧪️tests/🔄️lifecycle/🐍️.py | oracle | 2d552d36cc484ee7529c9e89698340139795e368c31b40e2d2e9cd0f326b0c8d |
| 🔗️consistency/🧪️tests/🔄️lifecycle/🔣️.json | oracle | 9c76102cb8817797edb30f41a45e9a2bff78fcb7faeebabcdb1a9017c1fa480c |
| 🧩️models/🐍️.py | production | 68301a5fb5072905b2ca765396d3bae91eaab3fdfc7042809d8cbc3cb6545454 |
| 🧩️models/🧪️tests/📏️footer/🐍️.py | oracle | bbc45a27d875732380003fb80c67677a4ddf3891da46c017b35cb2bee8815847 |
| 🧩️models/🧪️tests/📏️footer/🔣️.json | oracle | 8913084a4946dd943ba74f6dc6b056ca58d95d4d1c3fd0baba834585b0fb4378 |
| 🧪️cases/🔣️.json | oracle | 4f0cd53a158ec281f887eb962c979345ffce67566e8b7986db4db88083dc7092 |
| 🧪️tests/🎭️presentation/🐍️.py | oracle | ed8d085530183570b6b3346044f03acbd9cb2a32fb6ade0bbb581157cce8e604 |
| 🧪️tests/🎭️presentation/🔣️.cases.json | oracle | 2b5006f805dbb32bd33b9a14651b2c4e12fdc2b4676030b986c51c5eca3cb166 |
| 🧪️tests/🐍️.py | oracle | 70ee1ffa6435e84fb1d166cb34d97f4d27cbf20d6182845caca97bf930129285 |
| 🧪️tests/🟨️metadata.mjs | additional feature input | 487f52f9041e91d11db3c0aa361ac68eab65adfa4326abaab426abb924426bee |
| 🧪️tests/🧩️chapter-source/🐍️.py | oracle | 2dab892df719eef10d69f0a191f410cbe97658a791cc388c8bd776d8d7979784 |
| 🧪️tests/🧩️chapter-source/🔣️.json | oracle | 79ce833192415cdb5cd03551ce68542587c252967072955c93e8e2810cd0c0fe |
| 🧬️objects/🐍️.py | production | fcf271d4ce7c58c768d4fd4c26face14125b13061ddaddbfec1bda1dd41719a5 |
| 🧬️objects/🧪️tests/🚪️retirement/🐍️.py | oracle | f1ea248f8f7c02a736cf4fc60d3376744603e200162bbc0cfd08c889f077cd18 |
| 🧬️objects/🧪️tests/🚪️retirement/🔣️.json | oracle | 019f0e11daf5990a42c6eb73aeda3db64c31cedca5b22a0b5cbf36cea053fba2 |
| 🧱️model/🏗️facade/🐍️.py | production | 4734cdb30638d6d8d819842d68dde2d75971059cdff709c9133ca6b56008d582 |
| 🧱️model/🐍️.py | production | cdfd7872fd649b5bfb747e76ea979d2111520d9d137337627a53400710a1ed78 |
| 🧱️model/🔌️exchange/🐍️.py | production | 3d7d432b28f977d4d4833c6a1c29247a0a8dccd3db638df6d1e6e10109c0bf0f |
| 🧱️model/🔌️exchange/🧪️tests/🐍️.py | oracle | d753ed0fa4bcabd01c226bb6be5bc08f20e94744238b5e8319f893420c8b7d98 |
| 🧱️model/🔌️exchange/🧪️tests/🔣️.cases.json | oracle | a8d395ba00fbdd78395479cd1888e576cca505f4da665ebc8799e86fe68d6d81 |
| 🧱️model/🔌️exchange/🧪️tests/🧵spans.json | oracle | 0713ae1062a4a80067664641eed48defee101aad6870f497c22eb2cff526ca33 |
| 🧱️model/🔌️exchange/🧪️tests/🧵spans.py | oracle | a4abc0e7a9350d012abc1366a275ecd2bae7c0b644ae52c210fa06968701cfdb |
| 🧱️model/🔌️exchange/🧪️tests/🪟️fillings.json | oracle | c67acf32c47d47197e0c7af36d6a1d395185ae9044df0d4bee6f91bdfd27c7a9 |
| 🧱️model/🔌️exchange/🧪️tests/🪟️fillings.py | oracle | f1b6921b8a1d4602146d88e6796f1e041d1709d0b9414e86630453db9595e3d7 |
| 🧱️model/🔣️.json | additional feature input | d2f3c0d1bb97c93836fb61b53adfe94756c59c31217585899decf09cda4f59f2 |
| 🧱️model/🧊️profiles/🐍️.py | production | 13fe106796eec3cd64e173975ad525f401b7f6a7d53de2dd9777b24dd28296a2 |
| 🧱️model/🧪️tests/☀️rooflights.json | oracle | 2236d341126be06546575c68a22689a0babcbd1cc70a85503c081097d978decb |
| 🧱️model/🧪️tests/☀️rooflights.py | oracle | d0c0f017ea11b9c9fa96849084c9450b1cd0ded0ba449cd06f19a21dd1b70371 |
| 🧱️model/🧪️tests/⚡️partitions.json | oracle | 7f5b103fdd3d5e6c177481b0aa74efc3db0eb3ffe530b5bde67dba949f34868e |
| 🧱️model/🧪️tests/⚡️partitions.py | oracle | cd4cff05439d2059e3242e06e437b0eb3f50f1488c3cb250ac0692487d00bc24 |
| 🧱️model/🧪️tests/⚡️slabs.json | oracle | 2cb8f5faaa9e88c7511e522256e8648bc484aa04c2f7cee6bc2eca41985ca62d |
| 🧱️model/🧪️tests/⚡️slabs.py | oracle | 550df79b6419a3246d9ee377bf2c05667d6acc7af367689d5ee7ec221daaaada |
| 🧱️model/🧪️tests/🌀️screens.json | oracle | f33334c7b3d4c981d0a9b2ce5cbf4295a6bc768818adad0258e95241abec3ae5 |
| 🧱️model/🧪️tests/🌀️screens.py | oracle | d9fdb60a7e53e1dab55a0f7a911dc05f1cf80213d62c1f4c1907fd151b8a9aeb |
| 🧱️model/🧪️tests/🌀️wall-routing.json | oracle | 3eddd2d3954166c72ee6ec0ec1f9b0feb3e6901d21c364bc93f668f79ac1fd34 |
| 🧱️model/🧪️tests/🌀️wall-routing.py | oracle | 65e69925a30cd5779548835b86bab2e67b44fd2bdf52fa412eb879f67a313513 |
| 🧱️model/🧪️tests/🌬️garden.json | oracle | 16a0186a33cea4128168c3be2ef400f570f8893f25448d40a76e0acb5cd5fea2 |
| 🧱️model/🧪️tests/🌬️garden.py | oracle | 9731898f8319d6f9dc28637e6c7f06ce92ca55510d3465cfa942c184601a0d77 |
| 🧱️model/🧪️tests/🏗️beams.json | oracle | d3bdf89f7b656e8f562148e6286740c0cb13fd77e887eeac3a97044595fba61b |
| 🧱️model/🧪️tests/🏗️beams.py | oracle | 39a3b7794a99d5d383f91c1a6f49b9dae7cbf2a010c7a01c10518160e6033390 |
| 🧱️model/🧪️tests/🏗️bearing.json | oracle | cc73b437eb9edfd90c137b45e60e280187710a83a7920d9fd056fba23c900f39 |
| 🧱️model/🧪️tests/🏗️bearing.py | oracle | bcb31cd19a818763b02e0f2821b179c8339448a4238b519481b80ef5e5613e0b |
| 🧱️model/🧪️tests/🏗️facade.json | oracle | ad3d7bc13f59a018502cc7bb2155c570813d0a0dd6b7c4c3989a3e4910ae55d0 |
| 🧱️model/🧪️tests/🏗️facade.py | oracle | 12d0ee1f6cbc01d00adab10efb4cecc57d60e96bc0360a2c17e8588548b37de7 |
| 🧱️model/🧪️tests/🏗️foundations.json | oracle | eccd8299c8453b8ffce9017b2b9a5a5979293d340505e6f40f9d620a5dcc26e2 |
| 🧱️model/🧪️tests/🏗️foundations.py | oracle | 42f54d1942f402787619295a830133edfb9fc772c29221a49dd50ffb065117d7 |
| 🧱️model/🧪️tests/🏗️joints.json | oracle | d015dbb01a6003dffd63ff79ceaf826afb7fba2900377ed1f0879a78393ab053 |
| 🧱️model/🧪️tests/🏗️joints.py | oracle | bae6c6bb4d100bd754a39c73a7e62fbfe17e7147b867a0836bd6d3f344144c70 |
| 🧱️model/🧪️tests/🏗️services.json | oracle | 072b21cfa6b6ffeb13cfeb558a5a0b99ba6800a463d520cab9105447db554a30 |
| 🧱️model/🧪️tests/🏗️services.py | oracle | aa05a713a5067a5b8107cb6062a7b2615d8304da9ae7bc8931e6b8789dd6289e |
| 🧱️model/🧪️tests/🏛️architecture.json | oracle | ab6fea517d95b4609ff8298b787e06db3db33eb2057968d8cc1ef080939c72af |
| 🧱️model/🧪️tests/🏛️architecture.py | oracle | a39f7390e6c82095525e8665452aef0fb08dd2ade157ff521cc1e38c3b92d201 |
| 🧱️model/🧪️tests/🏛️invalid-relations.json | oracle | 1186bcb31b68ecc8abc7f6ff72c3ae505848f30c763124a49772e57b38e0c159 |
| 🧱️model/🧪️tests/🏛️remodel.json | oracle | 9d84c6593b92142819c53d254924fe697497f0bf59fa295418cc671fd650e347 |
| 🧱️model/🧪️tests/🏛️remodel.py | oracle | d4c072350019cac9c600912840bce29ff5fba15e5949fec7c71f1afa5eeb4151 |
| 🧱️model/🧪️tests/🏛️suite.json | oracle | b40f9f240882b393be9062ed3d0317f0bada14cf918f9f3a7aa60202d5c3759a |
| 🧱️model/🧪️tests/🏛️suite.py | oracle | a97f3069ee76896953549e9dfc467daa004a1b2f77313f14535ca3998bc8b9fd |
| 🧱️model/🧪️tests/🏠ground.json | oracle | ad0235021748106eb4a62ae22caaee08aca46aa2615b09a2c8c0a97c2e366aa2 |
| 🧱️model/🧪️tests/🏠ground.py | oracle | a69a8991f2a8f3ce9eea83ec31171bf2daf93d9677f2f55503d8ef4823e571a9 |
| 🧱️model/🧪️tests/📍️placements.json | oracle | 0fa7aa73f3ab16e8ed6c2a19ddd87fe0b3578104291abb43fbbb34c7335ebf9d |
| 🧱️model/🧪️tests/📍️placements.py | oracle | ec9eb3ed40e7fcc0d602b7772c3e2904f72448c1767f966125fec72344a2d3a4 |
| 🧱️model/🧪️tests/📏️columns.json | oracle | e16238773aaf5bf1e463b5f5fbbc54f9cc6dcf5b63ea0749e18667000e6cef2d |
| 🧱️model/🧪️tests/📏️columns.py | oracle | 81a0afa2d36e0753094802ee082964c5a2735915a0808464e3794fcd7ea96f61 |
| 🧱️model/🧪️tests/📐️representations.json | oracle | 7705914c23eae3c8fb6f156451102d47c0a31fd008a953f08b391d514593461d |
| 🧱️model/🧪️tests/📐️representations.py | oracle | 76da03da756000180e40325add67d9fdeb4f23ec6a96f4178297c35c0969b72c |
| 🧱️model/🧪️tests/🔺️frustums.json | oracle | 019df3ccd35cc46a7fb12698d36948ff268d5ead5f3b6ea7b121db6c4f85e6ad |
| 🧱️model/🧪️tests/🔺️frustums.py | oracle | 825d454a3c68583cd2391d40741d53a87d83ca8507ca64eed49af2f1deb79dd4 |
| 🧱️model/🧪️tests/🕳️cellar.json | oracle | cb5ebb11195d9de76da32d1a9929d83947a23a37ae1cae110a402b804a23cfe8 |
| 🧱️model/🧪️tests/🕳️cellar.py | oracle | 6a270516df8c8a7edf08b6ccb6d218c3e06272403bd7bfd032eed88761af0c95 |
| 🧱️model/🧪️tests/🕳️openings.json | oracle | 92f7068335308ce1cf7772a91f8573f8b941d8617eb52d028459c3e1e1b3997a |
| 🧱️model/🧪️tests/🕳️openings.py | oracle | 63dc126db1d7b22d1622f104799b82ffe375a83ac20bbadd98bb4785f352a439 |
| 🧱️model/🧪️tests/🕳️profiles.json | oracle | 6c9ca7955994360feb5879be4b035ddc71b3435dd22e07d0d48e279ed6de028b |
| 🧱️model/🧪️tests/🕳️profiles.py | oracle | 868c7c29c035e0027c969f45db0e412d93f3f2814f4e1ea4bc4f5be6764099cb |
| 🧱️model/🧪️tests/🕸️mesh-clipping.json | oracle | 26bed2f22a65cc069728f5a0c70705f8ce8ad947b602bc26f0ecb51ef6b254d6 |
| 🧱️model/🧪️tests/🕸️mesh-clipping.py | oracle | 425d422a4fa92a4aa95b5e0b8fadea3c665f57908c03bc36cf528f644f6b6353 |
| 🧱️model/🧪️tests/🚗️overlay.mjs | additional feature input | 375a3454b09ad7836e531238e9079c2bbc57ec3fbe1d87239cfea093cb15304a |
| 🧱️model/🧪️tests/🚗️overlay.py | oracle | 837e7eabe0f28a45ffcf3aef18179518ba99b787e14dac2edca7797d70b3b950 |
| 🧱️model/🧪️tests/🚗️overlay.schema.json | oracle | 8326b0e2fa3423c573a50f16f1f1ebd9e9807a8b1d89bbd47de1c47f56b0b07a |
| 🧱️model/🧪️tests/🚗️overlays.json | oracle | 7238a30a49e18dc78c66273f059f6881c2e6ba1b5322d598a2d039ee7326d313 |
| 🧱️model/🧪️tests/🚗️parking.json | oracle | 35d5a7b65a05addb97ff1287c6d3ac95ed33a15b6125390f685cc1ef036bdf68 |
| 🧱️model/🧪️tests/🚗️parking.mjs | additional feature input | af57e14ae6e07cda9a9f575fe1a8cf79d57abc577b848e4863df840f8d719b00 |
| 🧱️model/🧪️tests/🚗️parking.py | oracle | c2145cc65aa435369ba73a11747e8eb032402b2455dc8c2716200ee6bed9a5d5 |
| 🧱️model/🧪️tests/🚘️profiles.json | oracle | 4ab19147155d8806e7897805bb914a3fa500a01d159a53236fd50ca3c203da58 |
| 🧱️model/🧪️tests/🚘️profiles.py | oracle | ea21d92da8fed12c7b9ba9c7e83923c097a460b0f9c7b90ff0654fb35564b5f4 |
| 🧱️model/🧪️tests/🚪️garage.json | oracle | cec44355b25152bd7f0dd95c088a59b0cd6b664236d88508a996fe2ba0b8d4d0 |
| 🧱️model/🧪️tests/🚪️garage.py | oracle | f61f97307f9fee3aaaf85645227510992a1aadc940dbe0165de6a90745ad775c |
| 🧱️model/🧪️tests/🚪️roof.json | oracle | eb32bfd10282eadc5140a9625a68be83e9eaa0b30f2b697e6f2b2738f6264489 |
| 🧱️model/🧪️tests/🚪️roof.py | oracle | 302fdf31192b96dd7f9e2a9c133a97658bf1fd745f2298c71104799332f9608c |
| 🧱️model/🧪️tests/🚬️chimneys.json | oracle | c4374f49f22fc833fd7c6777f1f2861d4842809fb453e6f190ac92fbfa15f234 |
| 🧱️model/🧪️tests/🚬️chimneys.py | oracle | 2a735359d4c992d5bf0a231c8ffc265c79128dc873a274a0ea074180bd27f26b |
| 🧱️model/🧪️tests/🚬️prisms.json | oracle | 804d0aa2ce178b7a1bc44ae458c5c3181be1cc0d1c5271778d4287b382edaa22 |
| 🧱️model/🧪️tests/🚬️prisms.py | oracle | c7f5889c69acd8666ca1ab3f0ffe50c3776b9faac66c34a083e4aa6cf5e29139 |
| 🧱️model/🧪️tests/🚰️services.json | oracle | 34c9f0efb315fac9071003b933cabf77eef19efe13a8275961640bbf74c8ab93 |
| 🧱️model/🧪️tests/🚰️services.py | oracle | c3b91fc92c66002328b08e3ae40f8204226e28d8e19aa12e8e995dfcfba75ef8 |
| 🧱️model/🧪️tests/🛁️details.json | oracle | 57b17486e2abf136c7d90593448a75c05d5818d89e64fa674839b1a18d39b747 |
| 🧱️model/🧪️tests/🛁️details.py | oracle | 6f9f4a4883c5613fa2c430c7ec4a8b68a610a5178ff200ba8153021a7f5ea170 |
| 🧱️model/🧪️tests/🛝️ramps.json | oracle | 7320cf26fecb582a5efd52ed15e11e4a9c43febc3b92fe5a5ecf1dad79b277d6 |
| 🧱️model/🧪️tests/🛝️ramps.py | oracle | 4c9b0cd46620c48a94c0ea3f4e642583daeddfd68e72f6e2081d6a65597b2daa |
| 🧱️model/🧪️tests/🧊️sections.json | oracle | 9c0d39ead9641e97ce3b3f1f0fbd5524ee2453680b54d7ac6a7add756045119f |
| 🧱️model/🧪️tests/🧊️sections.py | oracle | 67500af0a668a61067dc6c1bd9696dffa05f7e4a8c8c6dec32d01d3c0508e048 |
| 🧱️model/🧪️tests/🧬️loss-selector/🔣️.json | oracle | 9ee6eaa8c2ad1f27aed756d20f40b1d977292911157eca154b1fa6c5d3aeaeb6 |
| 🧱️model/🧪️tests/🧱️walls.json | oracle | 4b9d5c1b44bb701cfde7c335b87cff74a886b622f6f8995ffe464816b5727a1d |
| 🧱️model/🧪️tests/🧱️walls.py | oracle | 0942c3ea687171f7e8dce131f29b0632db1c67c75aa9eb663c1f51441d6edd7e |
| 🧱️model/🧪️tests/🪚️joinery.json | oracle | 8cf79d12cefbb7206222bc193a6c06c1f47bfa9dd87f78f0b8252236ef7f0ced |
| 🧱️model/🧪️tests/🪚️joinery.py | oracle | d6f8364bbbca0edf5f93010145c4f3f5687b95dd2b03f672d3e54f65ff77dfa6 |
| 🧱️model/🧪️tests/🪚️ownership.json | oracle | 7b803151a8993777f8bdede8c0f437eb36076b2a321f2bf9442a1235bb683fa3 |
| 🧱️model/🧪️tests/🪚️ownership.py | oracle | cc3c0c84ec5f961a00d7cdb4fc46ccda50699d2c071da2f1f30cbfee209aa8ec |
| 🧱️model/🧪️tests/🪜️stair-parts.json | oracle | 6f57388e1bb5684862a13776560d7f1264b17978abc75e3f4db93df911f980c1 |
| 🧱️model/🧪️tests/🪜️stair-parts.py | oracle | 8829ef58097ae051dc9f4c5cdd0d3cb8c82d6f111c1df490bd29fbcbd15ab2da |
| 🧱️model/🧪️tests/🪜️stairs.json | oracle | 8630090e9b6eb7a7826730a4c112415225720d274da8286ca9dc00ef84c8ef73 |
| 🧱️model/🧪️tests/🪜️stairs.py | oracle | 090d1d0e1cd48e5fc0766034e2e816d61a214e7563239009bc451732208b1816 |
| 🧱️model/🧪️tests/🪟️curtains.json | oracle | a6f59ad8e11b3246df4ab94be1932425c30fb406b1f40723c189e2bd582b42cc |
| 🧱️model/🧪️tests/🪟️curtains.py | oracle | f752e0f5b08cb27a460e04b19ab7343ff51395de8e9cc8a2479ffdde09b71ea7 |
| 🧱️model/🧪️tests/🪟️fillings.json | oracle | 1a828f407a927ce8a76dd7fc5db55174242832ba7e04687ef8dbace5ce480cce |
| 🧱️model/🧪️tests/🪟️fillings.py | oracle | d0f9432843b7d99b286b3fdbc7572d2257fd71dd7c9108e2cd52e2baaa5f694e |
| 🧱️model/🧪️tests/🪨️regions.json | oracle | 28d68615030ef5a866c113db06e0106a29dc6d39eb6a87d90f0b8ad8c30a5e77 |
| 🧱️model/🧪️tests/🪨️regions.py | oracle | f9b59985e2403999170202d0d6879e31f44f51446bf76b67d09db7e4de9bddb2 |
| 🧱️model/🧬️schema/🔣️.schema.json | additional feature input | 785dd04d46b3ea892390a94f1edcf09840d42fd31d60d1baae33270d5f01560e |
| 🧱️model/🧮️analysis/🐍️.py | production | d8f3c7f5620ce384d15486c076bbab2795c66af43f84ef02e886a0599a52ea6d |
| 🧱️model/🧮️analysis/📏️distance/🐍️.py | production | 86c1b4a2d972c800a803685a795dc058982f58eb4cc306bff09cd8b3ea191d6a |
| 🧱️model/🧮️analysis/🧪️tests/🐍️.py | oracle | ff50623fc89a1bb31a48f3910a00d83009a872c48cd13f5530f5011260c96adb |
| 🧱️model/🧮️analysis/🧪️tests/📏️physical.json | oracle | 746113bb61725e315d71c87cf66803eba4adfe1f9a506cb87b60275ed819ae8b |
| 🧱️model/🧮️analysis/🧪️tests/📏️physical.py | oracle | ed64ec6d644dbc2662ea9620cdbb888911baea5497cdf028f039afbdb0de599e |
| 🧱️model/🧮️analysis/🧪️tests/🔣️.json | oracle | e199eb2d85f2fa037b3cb33c0949c41f1ba99456de083577bf52bb055329bcc2 |
| 🧱️model/🧮️analysis/🧪️tests/🕳️clashes.json | oracle | 01fa8b1c995627ebd09d4bb217cd6d2dab7c84bbed068d7cd79985391b1add51 |
| 🧱️model/🧮️analysis/🧪️tests/🕳️clashes.py | oracle | d3426b21c1fb71094ad4998b7d7ee58c6816620a07b8526ca23bc5ca232d0eee |
| 🧱️model/🧮️analysis/🧪️tests/🕳️passages.json | oracle | c57ae7b934e6ae5186ff8b959a8211c9a60e0a14932c8ff7b0beb438115d4ae2 |
| 🧱️model/🧮️analysis/🧪️tests/🕳️passages.py | oracle | 6511eb2cdd353a5af79ab5e24b51bed2e442d94cf0f5c06e952d3fc1db2121ed |
| 🧱️model/🧮️analysis/🧬️schema/🚧️clash.schema.json | additional feature input | 69cb431cf41fe4cd65cd1cbe2efd66354e15eca0f8a1731367b23d132eaacb32 |
| 🧱️model/🪟️curtain/🐍️.py | production | 7e85b6a2527a21ef81cd2b27a30aaeebf1cbc8147a605361215566cdcb9e5aa3 |

## Proven In-Run Differences

The completed154-method core recorded2production-file differences: 🎨️visuals/📐️drafting/🐍️.py and 🎨️visuals/📐️drafting/📏️geometry.py. It recorded6oracle differences: 🧱️model/🔌️exchange/🧪️tests/🧵spans.json, 🧵spans.py, and 🧱️model/🧪️tests/🚗️overlay.py, 🚗️overlay.schema.json, 🚗️overlays.json, 🚗️parking.py. Exact hashes and attribution limits are in clips-final-core-r64.md. These are in-run differences, not the complete task working diff. The final54-method and physical callback runs report no production-code differences.

## Documentation And Retained Ticket Inputs

Known repository documentation work outside the recorded Python/JSON manifests includes clip/README.md and regenerated English/German accessible transcripts. Transcript paths are not present in these manifests and are intentionally not invented here; their exact paths require the existing narration/collection owner ledger before ticket close.

Final durable reports saved by this agent: clips-final-preparation-r63.md, clips-final-core-r64.md, clips-final-repairs-r68.md, runtime-registry-final-r69.md and this ledger. Existing roof-door-integration-r49.md, curtain-ownership-r56.md, metadata-source-ids-r60.md, step-token-span-r62.md, clips-registration-plan-r44.md and historical integration reports remain retained. Root final-repair-proof-r68.md was updated to reference completed fresh verification.

Retain input scripts including python-validation.mjs, clips-final-baseline-r33.py, clips-current-cohort-r33.py, clips-preparation-r11.py, chapter-native-drawings-r24.py, clips-final-affected-r68.py and clips-final-callback-r68.py. Disposable generated outputs remain under the ticket generated directory until the parent completes composed-proof reporting and cleanup. The ticket remains OPEN.

## R72a Test-Only Supersession

The later R72a composed slice updated exactly these7previously inventoried files: 🧱️model/🧪️tests/🏗️bearing.py, 🪚️ownership.py, 🪚️ownership.json, 🕳️openings.py, 🕳️profiles.py, 🏛️remodel.py and 🏛️remodel.json (all under the same model test directory). Final method/helper/fixture identities and exact pointer deltas are recorded by features-r72a-continuity-final.json and fixture-pointer-deltas.json. No new repository path or production/source change resulted. See feature-tail-r72a.md for146canonical passing IDs composed from preserved133+fresh12+fresh1, with historical failures/stall retained.

## R77 Physical Fixtures And Native Furnishing Support

The recorded path union gains3checked-in paths: 🧱️model/🧪️tests/🧺washfixtures.json and 🧱️model/🔌️exchange/🧪️tests/🪑furnishings.json/.py. The union is179paths; this is still an inventory rather than a Git-modified-file assertion. R77modified existing model/🔣️.json, model/🐍️.py (_by_class/information only), model/🔌️exchange/🐍️.py (BODIES/step_excerpt), source schema, main tests/cases, details tests/cases and parking.json. Exact source and classification deltas are in fixture-repair-r77-publication.json and furnishing-native-r77-transition.json; only parking signature/physical obstacles changed in parking-publication-r77.json. See fixture-repair-r77.md, furnishing-native-audit-r77.md and parking-r77.md. Source/producers/tests are frozen for the fresh R84/R85cohorts; the ticket remains OPEN.
