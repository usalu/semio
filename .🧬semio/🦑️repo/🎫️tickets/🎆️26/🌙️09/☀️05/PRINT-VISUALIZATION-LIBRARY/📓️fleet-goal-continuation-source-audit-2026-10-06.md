# Goal Continuation Source and Native Gate Audit — 2026-10-06

Read-only audit in progress. No tests or product source mutation. Exact historical294 frozen source table rehashed; 294 entries, 122 differences against current bytes. Detailed findings follow.

| Changed Historical Compiler Input | Prior SHA256 | Current SHA256 |
| --- | --- | --- |
| 🧰️framework/🔨️modules/🏃️process/⏱️budget/🧬️schema/🔣️.json | 0fce193ad5893066dba3902d8cd83d53a670b9199aeb4021174ccbbad3a21f8c | 7ABEE010126A44056CBDEE1717D6BC24CE14353D66AD61FC36092B3611E988D1 |
| 🧰️framework/🔨️modules/🏃️process/📦️artifacts/📤️publication/🟦️.ts | 29953b7d7cb329020a0b8cdce419f89dce235aedc956cecb5fc0f761b065df16 | 92F4A6CC19C42DF333CBC6D0FD9F94EE72B6FA9EFA97688CDF554FC3ABC65DC2 |
| 🧰️framework/🛍️products/📓️print/🔨️modules/🎨print-design-token-paints/🟦️.ts | 87630cd9f36a3b34223a054e06bf5ee7794cde0cf46b1d733cde800be53f54e9 | C19D9D54765CBEAC0600A6E3154D073DA0F7C86FF59E54F7DC42987579509459 |
| 🧰️framework/🛍️products/📓️print/🔨️modules/🔤print-font-catalog/📏️metrics/🟦️.ts | c2de06b5dbe7b1d3cc6e7434f501f7d8739639c3cffcbfe5ef079f825e5494a5 | 5259989B07842FFE5FB2FBB46B3BBF8BDCA676A8152688C29C1EEC5883FA2364 |
| 🧰️framework/🛍️products/📓️print/🔨️modules/🔤print-font-catalog/📏️metrics/🧪️tests/🔣️.json | 9f411c69c93c6e6745f0f64ea29f602cf3c3eade37d35df7fa8900c7075dde4c | 1567B28CACA386031B9582B9862BC6B342A637CBE64C4B39A359F11604D67330 |
| 🧰️framework/🛍️products/📓️print/🔨️modules/🔤print-font-catalog/📏️metrics/🧪️tests/🟦️.ts | b09d41ecbd44aaf214ddb8e4a553047a4f82c09cfa93e5c420bd712aee890e60 | A63AA2E15E7B02FC480784CBB2A3BC50A6A26FAC1A7353067ED907D9043D5D31 |
| 🧰️framework/🛍️products/📓️print/🔨️modules/🔤print-font-catalog/📏️metrics/🧬️schema/🔣️.json | bfea112a059f6b56f2b56114fc1b7a2c61442e0ced972fff37c0ddf284e23bcc | 6E653880D85FED7C13E9AB2CD7EC4743812AEDB91131A5250072829906BC7CD6 |
| 🧰️framework/🛍️products/📓️print/🔨️modules/🔤print-font-catalog/📜️script.ts | 71dfc08d9c87113aa71d7b393584778245c4d7e63fba869b9bea0e6df9f63fe5 | 67CD9777008BFBE516456E6E8E630F8ECDFB99200E5DD9ED7AFF07EE683EC9EB |
| 🧰️framework/🛍️products/📓️print/🔨️modules/🔤print-font-catalog/🔣️.json | b63eb4a1255760f5b19a02298da33c34bf14c21a4135db78eb344b02839ab451 | EBC06BDAF3DB76103C000006A9B74A40A19B52F80D3E86B9F519164C079FA8BF |
| 🧰️framework/🛍️products/📓️print/🔨️modules/🔤print-font-catalog/🟦️.ts | 93cbbd14ced00f2abd005e5e48b6fe6be4537f484cb2d09a9a33db931df59f2a | C83F46ED1509777133D88D1B6C1414538FE4FBD232E5E26F44424E8C24748683 |
| 🧰️framework/🛍️products/📓️print/🖋️latex/semio-fonts.sty | 33355d4403fbb0168b87763ce345e83efe3f146f152c9316925cf7a73887174f | 652634D20651133E775C0C7D5504FD4AEF80C0AFC8CBAE4F98AC54649A6EAC8B |
| 🧰️framework/🛍️products/📓️print/🖋️latex/semio-viz-catalog.sty | f3e1f8d02bd1ed5028da49120ff74289959e61c904b40fda7e3454880f4e3cb7 | F74190E87BDD3BEB6119C848B058FED19A29C68161C2BDEA839F675F1D7AD58A |
| 🧰️framework/🛍️products/📓️print/🖋️latex/semio-viz-data.sty | efb5e5d397af1b6bd21aeb4c70f61141aaff42dc1f2f03532ca7d4379cfb2cfc | 6551EAE29D19C287A975073A638D8C0047C3DFF97F831B8B03FA3BA066FBBA3A |
| 🧰️framework/🛍️products/📓️print/🖋️latex/semio-viz-diagram-concept.sty | d5bd013026a2d9b53eacd4c013d7bfcbf1a2701d31431bf167e1b83e2a126f57 | EA3FAB89FF2B5D7D135A3F9B65BFC4E6FB0BDC7BFE9CFEDD82EA895E0AA587AE |
| 🧰️framework/🛍️products/📓️print/🖋️latex/semio-viz-diagram-flowchart.sty | e2b5de23ccbf47b44d505d8e275197c1aa5b505b94e0a6e93711e1d0563e013d | 57C62803AD40E20B58E4D7609E2EB8217862ECBBC29F5AE8722C6B2471F6D6B7 |
| 🧰️framework/🛍️products/📓️print/🖋️latex/semio-viz-diagram-process.sty | b4e830d9a6ae944a7326a1088b1c78d15ed757f1651549483ded56db06594cb7 | E5048AF3A1E73C69D4243B391B31AD0E2E10980B1A0493A17149E026EB815D81 |
| 🧰️framework/🛍️products/📓️print/🖋️latex/semio-viz-diagram-uml.sty | 829fc63b9248379936862f038e739100d3d04e18d3f18d27a368c085f02db103 | 2EAB88A18DD518ACD44D799D8E35DFD1600DB8793A27678D28746995B7EDAAE0 |
| 🧰️framework/🛍️products/📓️print/🖋️latex/semio-viz-domain.sty | 268455fe5d3cb89bc49150b7859f055229397bf51fae3067e017566330bff6f2 | C45A4D255FBD2F6DB919DD14496C7E43347E936AD10787AAD97C87184FD575B7 |
| 🧰️framework/🛍️products/📓️print/🖋️latex/semio-viz-geo-routes.sty | 434e42f1ae4c623b81dd2478e98cd85c196ad99e99c8fea8ae2f569998bba3c9 | C8E4F59884222F726B253F66634ECB0A57E697E36FE944D2F2AEF9901FD06369 |
| 🧰️framework/🛍️products/📓️print/🖋️latex/semio-viz-geo-symbols.sty | 72e4edfe02e5c85c0087c862231e156c84c9abc2b46b8869bcf2d86a914ce0ef | 329A8E8E9695F2B594B35D605A94155B1A417D4823DE1A1AAE32C3C7893591AC |
| 🧰️framework/🛍️products/📓️print/🖋️latex/semio-viz-geo.sty | 04a45f0cf3c7b9ae61fb76b805c0e8f1adbfd5a44a3d7a78597a67f6adf0cc66 | 8D9396F48F7185922320BC04A6D3E7872363F3BC2F913E70969895D3DC1F8ACB |
| 🧰️framework/🛍️products/📓️print/🖋️latex/semio-viz-infographic.sty | ae1c0d5b4c592be5fb8627b2b5556406af531e1bce315a98f2755e8c4edf83f8 | 304A2DECD8F327C6DE5725A75F6B4B7ECADBD011B54CED79CE7368BF6EE2ADA8 |
| 🧰️framework/🛍️products/📓️print/🖋️latex/semio-viz-mark.sty | 5736881919461e54311e18f4f68b29da713c2aa66e16888ceae562b241e7112b | 96E728B24ED6CF5A9D7BD9EC75338A1B481C336EBE90E0FC8D754C34D619C246 |
| 🧰️framework/🛍️products/📓️print/🖋️latex/semio-viz-network-arc.sty | 67e16ad2f93f6b009b114c7c61c1445caf5a22487e2eea81306ada688ccb348e | 37AF71E7CDDE409A536750281E8F077FCDBCFC867B0A4A2EA6522C8ECD0D1F95 |
| 🧰️framework/🛍️products/📓️print/🖋️latex/semio-viz-network-graph.sty | e7e70bd06b1fa0c720423c58efee3537867394bf9fdc2b28545246c46016f4e7 | B2A029EFCFF61D0F605F8DB8890E541C8A44F3C28F7638E5430B9B7F277C109C |
| 🧰️framework/🛍️products/📓️print/🖋️latex/semio-viz-network-matrix.sty | e907eca8a0e9d065b59bb94dc145e49b9de5583fd28c8f364a288825f09d11f0 | 8772C565060AB7AAB6398D23B4ADE07F2502BA3989C12CA2981B0A39B8FF51CD |
| 🧰️framework/🛍️products/📓️print/🖋️latex/semio-viz-network.sty | 75b03be67391f39304f5ab1dee06013b9b6c23d49f9ececc5da11b0e8a82697a | E809E5A965FD9746BC383414861A27DFBCC73F477422AB2E8EE6322D76F9FC14 |
| 🧰️framework/🛍️products/📓️print/🖋️latex/semio-viz-plot.sty | 34b46a9d6272e2d6471f82c15f6c713834d828e4b1a0576b2c33b1e9e90f08d1 | 295E44E12D5F95320A9AD6DE297ADD48C4E256FFB8424BFD1C7AD92F8159D5EB |
| 🧰️framework/🛍️products/📓️print/🖋️latex/semio-viz-scale.sty | 6d7a3a1573129790321a00fe0b1cf7b1e16127fb8df2f063ab2a0e764b51068a | D34411B9A58E2F5952901615CECFDB24B6137A568BD438BD481F23F616D93783 |
| 🧰️framework/🛍️products/📓️print/🖋️latex/semio-viz-scientific-3d.sty | 745bc6317244c544518ac2c2bf758b785a4655934b44d8279e8c390817cf3866 | DB8C99134B447B9871B4F733B80C759FD6DA58F27D3973728EDC99299B53C4D2 |
| 🧰️framework/🛍️products/📓️print/🖋️latex/semio-viz-scientific-biology.sty | df3cb79f21ec2c097492df3851dcc72384f8f33703b7880ebe75fe96e9f5916d | F5D4B26C9D56841FAFD23FCD4A57115716BDBAE1987CA39A8602AB1AADF10C33 |
| 🧰️framework/🛍️products/📓️print/🖋️latex/semio-viz-scientific-chemistry.sty | 2dd740454e7640da268fa5f7db9027ddfea0c2f93065efaee6ea2552f8401442 | 5A42202FE3CD68876AC7D012E0D5A58A39D4E8CFCB15C10E690EFA7F2AF4EFF7 |
| 🧰️framework/🛍️products/📓️print/🖋️latex/semio-viz-scientific-engineering.sty | 05225652a914073f07f80383960ac8475d7d02f1031a436799cb37a107a447d2 | 05E69DF56F78DB851E6ADBD1D68169FD6A8CAE8A3D227CD54D0D6A2A4DA21EA4 |
| 🧰️framework/🛍️products/📓️print/🖋️latex/semio-viz-scientific-field.sty | 3087d2ff69c4cb68de6ad32b8b8ae8e7f8d5e23c13a9b503e23fd87f33feb62f | 6E4A7030C59EB9F599BB60163CE2F259F915C703F7A2F7AC28CCA0A798C2B4FA |
| 🧰️framework/🛍️products/📓️print/🖋️latex/semio-viz-scientific-geometry.sty | a8a7210d6f0159d1353ce169f482f389aefc4d6487e48364daae82b44058601a | 920B066AC245EEC99372909878DBC94D23B44ADDB3D695F87C76EF4128063DDD |
| 🧰️framework/🛍️products/📓️print/🖋️latex/semio-viz-scientific-mathematics.sty | 0d68f6393b8c80fa7dce37c7846cbeb0c593394dd864c1b2124a10bf639eeb01 | 1E7BE02378B379F65D396E037F75281C66C6E2E55B8A45732E642BD6C5849EE1 |
| 🧰️framework/🛍️products/📓️print/🖋️latex/semio-viz-scientific-physics.sty | 52dfb58beb621cd36eacecfb727cb9d41e9dc2c7509709fcd86eb986a59cb994 | 9BDDEDBB62F3370D4767BFF3285760374E783A2AEBB95D05A92FBF559174AE5B |
| 🧰️framework/🛍️products/📓️print/🖋️latex/semio-viz-scientific-signal.sty | 3f6d3b9b3433bec7ea2137959767f5363547ddb350d1e00158db979ea8dfe892 | 34C0AE85DC0EAC028AF74F45EA5532D855B7607A31BDEDDBB0F4D26AC5A163D3 |
| 🧰️framework/🛍️products/📓️print/🖋️latex/semio-viz-scientific-surface.sty | c646e80c3fb83c8ae7bd578134f95b871f4353be6f549d9e1002ab58a983852a | 5B0211083306625B3C1F621D54FBEA55967FEFA2B0232AFAB510B873CE942F41 |
| 🧰️framework/🛍️products/📓️print/🖋️latex/semio-viz-spatial.sty | 9f15cb7679ad20d6eda26a79d334a38391b361f8e9b9cb9d21716900936cfd1e | 39A6AA0F9DA9E755A46D04B86A060A11C8B621191114A8BBA82DB47BA23A3B73 |
| 🧰️framework/🛍️products/📓️print/🧾️template/📊️viz-api/🔓️viz-api.tex | ade4f49ed1febe6a4e769e400d45449e3eb0621462950642d607d5d16cf4ecdb | 89BBDE3A5A38621A6BD2B8FBFFD4CFD68266BD9AE360D25869CB4F8D47A6AB9E |
| 🧰️framework/🛍️products/📓️print/🧾️template/📊️viz-gallery/⏱️viz-2.tex | 8bb3f9e899e32be14ba096cd7e29a2f2b8d63dafe3aca2fb9bdb518b095ccc66 | 67CF782074D1B0226F0587CFF3B9C06B19820E67E51C1D8E76E1BE388B88354B |
| 🧰️framework/🛍️products/📓️print/🧾️template/📊️viz-gallery/♟️viz-56.tex | 8ae47b51f5cfaf0391f7f6561f35d2466c809a648df58e7568158e3d0f26a8fc | 4F3D39B9B3DFF167F6C367BBA1D0A07BCA0E193C4E04A4D3E333A3F6940A70E8 |
| 🧰️framework/🛍️products/📓️print/🧾️template/📊️viz-gallery/♾️viz-38.tex | 55a3c4c743b0a6e33de5fd0d61014e2a4420c2042dbe7948d7bfaf70bfde8e13 | 7F16B24069C22FDFF9E6E89CD4922CD8A99D02A510B6A06992CD446411524235 |
| 🧰️framework/🛍️products/📓️print/🧾️template/📊️viz-gallery/⚗️viz-33.tex | 4850d70320ae984d1d78c9e18f0054d1d8ea012999a33f9480d0e82b496e93a9 | A8F96A652F2773EF9D322BFF96D49A9EC26B94C5D219CF16DE6D43284038F82F |
| 🧰️framework/🛍️products/📓️print/🧾️template/📊️viz-gallery/⚙️viz-14.tex | 8f39beb6e6a0a16a6f1b5c0400eb07f6fcd6022cc4e275be042be7258ea1684f | 7CBDFF5607D1852E11B04E9BE8527BE8471C0B82847B4D2D30D9B1F6E039ADB1 |
| 🧰️framework/🛍️products/📓️print/🧾️template/📊️viz-gallery/⚛️viz-30.tex | ce8c0139b6f3dbe95b903becd29b0a892b9c3072b22194a2453600c894b96f4e | 66E9C10D6EA98708633F4041530324BDFEC57180B538193A1CB74FE7A2A73FF7 |
| 🧰️framework/🛍️products/📓️print/🧾️template/📊️viz-gallery/⚽️viz-43.tex | 51932db85c045963803a8705572c431fda7ccfeaa57ba34c9f1f904dfc927799 | 1C495B470B8B731271EA1EFE29519829456A894F257E5068A482FDC26B8E0A3E |
| 🧰️framework/🛍️products/📓️print/🧾️template/📊️viz-gallery/⛓️viz-64.tex | 5d36633b65fc0138704dc373bd4a0a2c6b98a6194bfbc3005cec9f3c85a4fee4 | 51C4E059F935B91758AA714101D96529196B70D8F38F11AB85456C08C29C6E89 |
| 🧰️framework/🛍️products/📓️print/🧾️template/📊️viz-gallery/❓️viz-23.tex | b8513fa1cee73ab46c5f30364e21d91a97b3311ac6abe17e82383d6c91aa6dbc | 4688A0CEA9F5BC40B8F855BA43766EDAD76D3F01302CACE2ECE519D078717FF3 |
| 🧰️framework/🛍️products/📓️print/🧾️template/📊️viz-gallery/➡️viz-27.tex | dc1d3c9ec324647a0107223f5909d1e9f0c00d211729607f3568181a264b7cca | 5FB1EF6E71BC02B930B967D0E11A16B6BD7A1311C1377A4A230F5F8D85C04302 |
| 🧰️framework/🛍️products/📓️print/🧾️template/📊️viz-gallery/🌀️viz-61.tex | a349ef7c05e2b9188cbb8bf1c8973992d022fb8e98c7a0a8089d0126572d44c8 | 5562153B475D37E4ED7D96CF381B995AACCFAF3FDE32880D4D282922B3D278E0 |
| 🧰️framework/🛍️products/📓️print/🧾️template/📊️viz-gallery/🌊️viz-9.tex | 7669e6d33f68a01f3da12e4d3b273098ec03a6114fe396bce9c1a8043395c94f | 910F993060877B45D4E7E2DB1D1B471C3A5414ABBAC7AA01DDCD2AC55D5A9D70 |
| 🧰️framework/🛍️products/📓️print/🧾️template/📊️viz-gallery/🌦️viz-35.tex | 3d7876d813720beaed37be61a4755f2b41f617f8bdb1bd6001a8066be1faac01 | 6B7995D0A112BFF976D6EAE1BCC8F97D0F6D3A6CF89E03BB51171A41268BF331 |
| 🧰️framework/🛍️products/📓️print/🧾️template/📊️viz-gallery/🌳️viz-7.tex | 0207abecefd2edeca4f4e11b1d6a75d538f60371ef919aede1db0eec0c783902 | 407ED1FA9020240D4C41E95A9F10E2E1162E991EF19F0DBDA5DA24EF1B944D75 |
| 🧰️framework/🛍️products/📓️print/🧾️template/📊️viz-gallery/🌿️viz-34.tex | 888edd38fee629742c2de0d4bcd7ddaba4b2d69c632e022fa13dd8f8dc20a5d5 | F6D53C5FFC3AF9C01108B08F50C1AB1FCA558D542B73FD5B0345C0E9E5E319DE |
| 🧰️framework/🛍️products/📓️print/🧾️template/📊️viz-gallery/🍰️viz-6.tex | af7b1981d3c39ac942309cc35380f0e21563bfbb0943b5026a425dfdfac19cbc | 585C800E9D3567FC7A5238F276737462E804D7E5437DE217DB9C5B67925405FC |
| 🧰️framework/🛍️products/📓️print/🧾️template/📊️viz-gallery/🎓️viz-45.tex | 88fdf0092e6a864910cbbe8d74ac4514220d40d8ef1371699162f724603691cb | 5FC7E03081F029D5FBBFE8B759F9827DD258745CD30C1678FE5B71A772E8730E |
| 🧰️framework/🛍️products/📓️print/🧾️template/📊️viz-gallery/🎚️viz-18.tex | b010a58ce01e9a186c8140beceec416db8f7e83247b782961d05c2234f42a255 | 46CC2A542301BE9D196F553D53528411A96FE2F3A2FBCBD6A2F4FB37728E4455 |
| 🧰️framework/🛍️products/📓️print/🧾️template/📊️viz-gallery/🎛️viz-5.tex | 9ac2f49d3a707b1dc839686b163b7902a9e34d81222fa46b2cb8d3cad044479c | 73E3FD7E43B073405701B1ED838836B506CBBA2600D1D54812F424BFC94C5C29 |
| 🧰️framework/🛍️products/📓️print/🧾️template/📊️viz-gallery/🎞️viz-73.tex | 83dc5bad9ba64957676556e020fbaa5b4f8f4971a403c62ee6aed0072d2e3f5b | 81C531F3FEA30BBF6CD3EB75AF3BCA74FB190B7DEE352BED1A8A515EB828BA7D |
| 🧰️framework/🛍️products/📓️print/🧾️template/📊️viz-gallery/🎨️viz-74.tex | 5faf76a46afea345c4d885d9d51682847d26c1a8ef854dd7a10f5fb58852aef9 | 5700F0BE059F0F2602FDE79696995B400C66F4871296C6938C0FE92BB604124F |
| 🧰️framework/🛍️products/📓️print/🧾️template/📊️viz-gallery/🎯️viz-24.tex | 940b85e1dcd1ee284d4d1cb35136279b31dbc303acb417f117e9cb2f10833522 | FDFB57EC482BF18CA6D95F68F44D9AF7DB62EA62F6C21AABDA0C3732ECDDC7D9 |
| 🧰️framework/🛍️products/📓️print/🧾️template/📊️viz-gallery/🎲️viz-41.tex | 530bda9b9490402860e003858c20b429cf21cb509e2a00c34195a7014f936617 | 7EA49954AB00D5C49055CEB2283A29740BB2732FF5E9BD5C21088D421CE6EC13 |
| 🧰️framework/🛍️products/📓️print/🧾️template/📊️viz-gallery/🎻️viz-3.tex | 089bd1e60a8cf6a9eec98349db8f4addfa8a3ad7c9083b77541a1dd291af4d6a | 8F39DED9F12275E128A7CF8A4D2F0A351E2FF3055E18690857E1D6C2BCCF55E1 |
| 🧰️framework/🛍️products/📓️print/🧾️template/📊️viz-gallery/🎼️viz-44.tex | 6511361d4a09df6e484fdeaf4db4c6a2d8985c50a0523fa167b8bf7de20407ae | B43A1758992787B5294CFA98DBB631260299034E968C45123EF8801F3069A43E |
| 🧰️framework/🛍️products/📓️print/🧾️template/📊️viz-gallery/🏁️viz-53.tex | 7c83d989776e75be496b35e011663623b424bf92961bd66b6274de074b962fdd | 8894E1641227EC8D6AE852C30D1EC7B0E069B6AC28CFBEED656FA6E382F81EBB |
| 🧰️framework/🛍️products/📓️print/🧾️template/📊️viz-gallery/🏎️viz-62.tex | 6f1a2c1d52a28ffb9c52601664703a2e89208578b498bbe6358414a7fe3ded21 | 7E6728F76E6CAAA05C4D118254DC588820248572A23DC9FA7D6A3332A01D3886 |
| 🧰️framework/🛍️products/📓️print/🧾️template/📊️viz-gallery/🏔️viz-28.tex | 38fda31c5877d394d1f86c9d1876c7be7ad60dcdc4afae2dd8056e657385b91a | 73C7F3528E2F6BE852B46D8EC7CA7EE58F4712A97E73886B6486A5836DB1FAD2 |
| 🧰️framework/🛍️products/📓️print/🧾️template/📊️viz-gallery/🏗️viz-79.tex | 7c428dadeaa572175fe57400ee9e90711c0b1b4335f25bc57e9c0ccbbd183114 | ABA3F71BB7B7029B4E059879416A0E18C035DA86651A667F1062EE28A82ABD39 |
| 🧰️framework/🛍️products/📓️print/🧾️template/📊️viz-gallery/🏛️viz-37.tex | ae882603b42d52a011da5c7952f66b6bbd205bf07b95829ef3aad6b517173cf2 | E3303B561A5B2AA0B6730977EB984C211E33EAA0D97216C080DDF569D579AC1B |
| 🧰️framework/🛍️products/📓️print/🧾️template/📊️viz-gallery/🐟️viz-42.tex | 41e83553dc2a74598482d8be6d70ff384b2659235eff2436e14a97b5df3c5fb4 | CB0F440C9FF844D33C567BFC058C4C4552CCBB2015182D7E5F19638163FC5432 |
| 🧰️framework/🛍️products/📓️print/🧾️template/📊️viz-gallery/👣️viz-63.tex | 6094bc788604d651c13f822ae3492a92a6cf05a648c967d23bae3bbdee515a5b | 96535E70E061DDAAE637D5D854742CF9A9830715E90394545768D84E5AF9FFE7 |
| 🧰️framework/🛍️products/📓️print/🧾️template/📊️viz-gallery/💬️viz-50.tex | 30467bf4010d0b01e4fe45103143c5b243f66036511774c106f66528f2b036bc | 849DA939C804F93C2C56709EE9044CD6170632AF6E31BAB53BAFFBDBCE335BFB |
| 🧰️framework/🛍️products/📓️print/🧾️template/📊️viz-gallery/💹️viz-17.tex | 4dcb6e7825c66319cff7b6f0ac85a8ae0886760c8a3d93b9be7ca3b1d5fd9a02 | 0A8A64F55918B965581A57D87D0B00CBFC9B7D910BF74CA84671D7C4E8A05DF7 |
| 🧰️framework/🛍️products/📓️print/🧾️template/📊️viz-gallery/💻️viz-57.tex | bf43cafcb3650c98f340323a08ec769f08ef5ba31cf21e28bd5e7dd913a9b7c8 | E678F466D6C9DC378157DA540D9428AF144A57F23234695B48CF9633D4F83C1B |
| 🧰️framework/🛍️products/📓️print/🧾️template/📊️viz-gallery/📅️viz-54.tex | 6614493b14be2f46166f7d7773fb7e9b9a07efe934f82bc71ccd3cd9ece58e97 | 57959647AD50B301C3A97FEB99DD21C244382B818F5E754E07B841D7BFBB0922 |
| 🧰️framework/🛍️products/📓️print/🧾️template/📊️viz-gallery/📈️viz-25.tex | 8e5545b5226fe2b5007e70bc1bb2d2249994fcc29c838ab4388d85385b560fd1 | 57E1FF37FB2C08C5178D118087931D1DA0A678A7D94E845C31102CBCEFD0F4E0 |
| 🧰️framework/🛍️products/📓️print/🧾️template/📊️viz-gallery/📉️viz-19.tex | 1ef784d24b8172b21b95614506f15811d967e1edd72c62ecfdf78b23fa5697ed | 79BFCDD77B34C049315F0AB2D1B7AE2C59C0F462756542953C8F4A54C42E0046 |
| 🧰️framework/🛍️products/📓️print/🧾️template/📊️viz-gallery/📊️viz-1.tex | b558fbde500b539cb89a51f3009d6265a4029c4e28419a5f58d4e4a5aaabdb60 | D4939BCCB2B65A964679AB383900C551927D81278D33E9EFADC50B18119B6FAF |
| 🧰️framework/🛍️products/📓️print/🧾️template/📊️viz-gallery/📋️viz-16.tex | 880ed63823f8f565c601526b7660b17a414ed842a445ab8ac94db7329f5e37b0 | 8D20CAB34C74CC151A37D925C0A94DB82CF077B339E4541350E725E9365C72F2 |
| 🧰️framework/🛍️products/📓️print/🧾️template/📊️viz-gallery/📍️viz-52.tex | e6bbc1d2f688d2269245395e53faabea9d3624c1fb0af32f7c54ec61a67fe710 | 3B47D003F4C878B391E07144B3234F060BD9C9D1FD90AA74908E7772EE1EF189 |
| 🧰️framework/🛍️products/📓️print/🧾️template/📊️viz-gallery/📏️viz-51.tex | 85c96cf7e7fbd22396e5c0b39600e5af1c97f6c4598b0d16d260065cd1215362 | 0DCA3F1A1212A97D68F9DCF284355326CF085600E3263145E4A66232BD199029 |
| 🧰️framework/🛍️products/📓️print/🧾️template/📊️viz-gallery/📐️viz-26.tex | c88ad295d9627de73465950544a15b8f0c168f9f527794143d58106e8a511ee5 | ED954E336D52458FC978675548F894DCF6587D3A6AB5F1301D58D20885D85A1E |
| 🧰️framework/🛍️products/📓️print/🧾️template/📊️viz-gallery/📖️viz-72.tex | 2989a8a475f9104cbc7b0c4c7c8b500eeef1dc704248cfd472f3eeaab770de73 | 85C9EA9D3A7AD56398E52B51C70F1EFA9ECBBB74A1B017836E41A96654F61B08 |
| 🧰️framework/🛍️products/📓️print/🧾️template/📊️viz-gallery/📝️viz-69.tex | b40fe9c3cb0e4e5ee8f3cab4a9d5598d3f35488d0471e60175350b3311161c8f | CB94F45C49C597D83C2184F9113C7F6689885DBF36EBE944F76A9EB59C1F5EDC |
| 🧰️framework/🛍️products/📓️print/🧾️template/📊️viz-gallery/📰️viz-46.tex | eecc4e8761cb0c83f73e9074ceea50dd6090e0be9dc7fdab7583d20e6f86aec6 | CEBBC215BC40412492C26CA59268C2EE4FEA9CF3A3C698D271E45D9867530C37 |
| 🧰️framework/🛍️products/📓️print/🧾️template/📊️viz-gallery/📶️viz-32.tex | 21a12d4dd93268e41575b248bc16405b858aaba245c4a606c3aa7bb672c381c9 | D460AEE26AF5C917AE1F3BBDC26C7CA58020193B96C977AB10D9BCC374D2E597 |
| 🧰️framework/🛍️products/📓️print/🧾️template/📊️viz-gallery/🔄️viz-78.tex | 3f923e0062af6c97739af2ff8fdeb37f6b52b272861644d296141ff37b527261 | AADE46C031BD3AA8967CF705F191CD64B82723EA17122855A49E7EDE830ADC77 |
| 🧰️framework/🛍️products/📓️print/🧾️template/📊️viz-gallery/🔌️viz-31.tex | 91a94a5e6dd0d392d0b945738a857635f31bb39ed0c273f12647954df684cb94 | 00E3822229FE96FDD85C0A42B0716175970A286FAA8352F055729CEF00ACEB18 |
| 🧰️framework/🛍️products/📓️print/🧾️template/📊️viz-gallery/🔗️viz-4.tex | 3b8a9d306d70b4b0449e8c3e75f434e71adb5edc21615a27707973ebfad9d6c3 | 079603E50D5FCA788F27C4D4039CBCDE212A4BB2A84EAA0B51DC46026B9192B6 |
| 🧰️framework/🛍️products/📓️print/🧾️template/📊️viz-gallery/🔤️viz-13.tex | 2843faa41a5dbec6533abbfc9d6a61c7a8bf84464fc23df87d356847cfc32f56 | F022BD69521E6AFFC999E88147A870AF469481AD41A5D9C81B2A8157C98A8118 |
| 🧰️framework/🛍️products/📓️print/🧾️template/📊️viz-gallery/🔥️viz-11.tex | 0c95cddc4fb59a9013e4c11289a65e62c5e3cb5c7d8a2793c7abd71897ed2eca | A34FC06C829D84AA80BAE832843244E2BA3F7AAF940C99171364E9A1FBBDD30D |
| 🧰️framework/🛍️products/📓️print/🧾️template/📊️viz-gallery/🔧️viz-36.tex | 265fa2ff2683af1d60a99672f6f1964a1bf7a2b2fbcb70d2aa8c7b042b42c332 | B2F156D6ED5E18E8D768BD22CEB59F8A3D88ABC4D2A6800E4C0B15BD12D7FB23 |
| 🧰️framework/🛍️products/📓️print/🧾️template/📊️viz-gallery/🔬️viz-60.tex | c6959d61524ab2d1f1fb7f3fb9f0688a9b4339020fac2bb7bb16fc099267bd7b | 953BB01B7FB5D38B86915678E018EADE672AF3994B6A39614024B246EFCC2562 |
| 🧰️framework/🛍️products/📓️print/🧾️template/📊️viz-gallery/🔺️viz-0.tex | 3910daab98532a5472fb05270ff84886ecd40c6aaaf4bef8fdbe7fa70b4d182a | 552B764B3728EEAF8CD962A30993FAFA471EC7B7F9ADA2045DFA22EE98020590 |
| 🧰️framework/🛍️products/📓️print/🧾️template/📊️viz-gallery/🕸️viz-8.tex | a5b2915f0500ef9c3b36d5db0578c2464c4c97b01458fe9f915f269294cc81fe | 00533916098E0FC1223DB119B0B82C63C3BECB0A50FFEABDCB58A82444842EE8 |
| 🧰️framework/🛍️products/📓️print/🧾️template/📊️viz-gallery/🖥️viz-15.tex | 21af7b87ea3e4d2d459723dfc9ba4a817626fd850f02827d6cdfced6c19770e6 | 1958324EE8F90D230BB01106814EF19CF60F2ABF0D9C3BD77AC6F608ECBEE3B5 |
| 🧰️framework/🛍️products/📓️print/🧾️template/📊️viz-gallery/🗂️viz-76.tex | 86dc088a48a2a2f7acd5c27be2d99cc38ce56dbde6f6ddc03b7952dd5cb020e7 | 9DCB2982D5BBB37BFCA778E7C0F2AE638824191957511E99F18783DAAB191204 |
| 🧰️framework/🛍️products/📓️print/🧾️template/📊️viz-gallery/🗄️viz-66.tex | f28087a26890317edf751cef005507951549ded5b0921a97f8f419ee3ba4fe4e | 7D2651ABA6C19FFF7AC72634197A221E0D7F6E994C88971B5C470F6805C6C40A |
| 🧰️framework/🛍️products/📓️print/🧾️template/📊️viz-gallery/🗳️viz-70.tex | 0c07fc4a48fd4b866d116a9f341fde9f44210ac1851c806586004344ea4236d6 | 4D826EB66F3DF78CC5C3144A3809FD775432D2C82FCC5434498BB1FD7506697F |
| 🧰️framework/🛍️products/📓️print/🧾️template/📊️viz-gallery/🗺️viz-10.tex | 925d67dab26e7a5dba000cb66c5422d5b8f923a6d6fff0fd7c2b9fb75d5c0177 | AA5A93124D49B9DD8482C0D17433D3D4B0AF17161E3A7E8A9A401E9E09327D9B |
| 🧰️framework/🛍️products/📓️print/🧾️template/📊️viz-gallery/🚚️viz-59.tex | 457d01084478088d3c0077c3f8d609ac6c229ad3a9a03d318f919b4ab8a871fd | C82692B044926015360C11F26C25A35AF78A5CF618C742A53CA7D0174601A3EC |
| 🧰️framework/🛍️products/📓️print/🧾️template/📊️viz-gallery/🚦️viz-40.tex | 55718eb249d4adbe2e869b75e77cfed6eaba6ee6bf1df860d5f34a4171960fb2 | F2B094A5278CA573FF927C98FE4E751D49EE6FEF8B8DAB7EFC744B188A2A4030 |
| 🧰️framework/🛍️products/📓️print/🧾️template/📊️viz-gallery/🛡️viz-58.tex | 4fa92a05e1ad34fe5b6ed2282ceef187f87b8fddcf5444ae46f55b857c0e7cb3 | CDDA4BFDE2A24BC3E815C62D92EC16AF2A66C4CE08A575C6F5F3F56C291D2ED4 |
| 🧰️framework/🛍️products/📓️print/🧾️template/📊️viz-gallery/🛰️viz-65.tex | daa9d51e634f5e451801aa1130d637067626e917c0f8b304665403212b8ef912 | E0F7016741607388A1A000D63E09EF809B0F6F78BFDF382708DE1A05A458830E |
| 🧰️framework/🛍️products/📓️print/🧾️template/📊️viz-gallery/🧊️viz-29.tex | 309004825eeb25e09c7ada27fcb434a0b9547e1bd71861a6327f92e4667c1e31 | 93BA2BA7C82C4B65CFC9B0EAD410EFB396340746D9DC5F27C3B120872108AC47 |
| 🧰️framework/🛍️products/📓️print/🧾️template/📊️viz-gallery/🧠️viz-67.tex | 2a389e3957dba913381fd60be8e02e34850c027954b2beeb8e32738a09afafa8 | 936F7D244B7AEBD7E7D6AD7817D16C22F82767DB97BF190B8459B489065D6E3B |
| 🧰️framework/🛍️products/📓️print/🧾️template/📊️viz-gallery/🧩️viz-48.tex | dc613baa30fd88ce3e3d79eb7463ec74e35d9f3482f661d2b8087069cb512577 | 3561C31ED64D69251833FBC7E755F199306653410336F22127F4AB644ABBBBC2 |
| 🧰️framework/🛍️products/📓️print/🧾️template/📊️viz-gallery/🧪️viz-68.tex | 394f8b2e1dda81409f9699f4ee6824f45f2145a822c87a26e13b66eb61fe1489 | E0AE175DB5D4FAA4BE9CBDAE780850918459B8A14416775BD8CBDE135CA15524 |
| 🧰️framework/🛍️products/📓️print/🧾️template/📊️viz-gallery/🧬️viz-21.tex | 072a2b2366ba069538faa8281a07effe35440b442793ced279208a95ec714c99 | 2A98F5AA7F279377B6E5B00CB074F1620B1512D577DCD768C4CF5921988EDBE0 |
| 🧰️framework/🛍️products/📓️print/🧾️template/📊️viz-gallery/🧭️viz-20.tex | e7ab719c360c0750f93222f256fb5c2570475054fd49b6c618e4e4e1d3ad71d8 | D767CC270616BEEEDAC29FD4FCFDABA3315F2696DA5357FC00DC808134AC2331 |
| 🧰️framework/🛍️products/📓️print/🧾️template/📊️viz-gallery/🧮️viz-75.tex | 79a8d3dd8e8859ca774b667751e4b97a605dac69d6ae0d2f8058534a9bf29888 | 4C54C0B322F1B234A2227C561F44BA4F1EBCE44ED1C1E9B9AFADAFE0C4DDBC42 |
| 🧰️framework/🛍️products/📓️print/🧾️template/📊️viz-gallery/🧱️viz-47.tex | 87ba0dd1c62f612576b3e72246efbd8ab8cb070cc3ae050c128f1c48b47c1b4c | 1B8BDF70037C1AC0E54F02603711B40C27D7058BC28C58653C8E6F8B1670FC95 |
| 🧰️framework/🛍️products/📓️print/🧾️template/📊️viz-gallery/🧾️viz-12.tex | 86c0bccecbad61a52a3ef7c17111d384624cd5824cf327dac6600966ae4f9292 | 59EBA9F441BACFB7EA27357ADFED2A91A9D75728C917D8CD23E034CFEF16680C |
| 🧰️framework/🛍️products/📓️print/🧾️template/📊️viz-gallery/🩺️viz-22.tex | 8fdec5ea711434dec15a62634373506bc1f01b3fbcb1a9ea51603f81b56c5bbc | C0C265F04C73640FA9C452C9AD66C7B80969C03978419DA682D0FBD56ECE9524 |
| 🧰️framework/🛍️products/📓️print/🧾️template/📊️viz-gallery/🪆️viz-77.tex | 507fb699f1c010481253bffdf4754a8138ebe1da7dab6a4c9f88c480a65015d8 | BE1B561B86681495A051649EA2ED5E6A3AFC8BE0487E9EB83DB7E3E530848313 |
| 🧰️framework/🛍️products/📓️print/🧾️template/📊️viz-gallery/🪑️viz-55.tex | c121e374fd9f3c4c62444bd230bc2c874e8174f586a06f90f8dbc4db7bfc1e8d | D1C99E416143E86F3ECA9CEEDB5A318C7C991102AA9046D2269638C3107A87BC |
| 🧰️framework/🛍️products/📓️print/🧾️template/📊️viz-gallery/🪞️viz-49.tex | 1261b467135c87b8df27df8537b448219dc4d521d279bbd9e28f30a15922ee4f | 02403A31106FD0FCF104A4D06FA0ACC4C103A6D35FF5FF7A573173317A3A573C |
| 🧰️framework/🛍️products/📓️print/🧾️template/📊️viz-gallery/🫂️viz-71.tex | 60b3e749a9c0849f66b7a7026d062937b5cd3b6a0c1fc97d63aed6a9b5c43bac | CFAC8858F79251FA48391F29FC2F9EE6CD7A97E5AF1E0975667CD30DFCFAF897 |
| 🧰️framework/🛍️products/📓️print/🧾️template/📊️viz-gallery/🫧️viz-39.tex | 364d2344e629ce3f7c284b7c8597c5a26380e905ab94d859474fa56ebf9b1141 | 4C35AC4DD59BE04D7B8ADE43C4C6F309390005DF6A140ACE42CA03FE91689F9B |
| 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🟦️.ts | 4c9334d817522a8db27f7ac2b82029aeb329aab84a888889dbfceafcda80db7c | 64F0AE4CDB39001668CBC70C55CB0BDFCBA865CD54D5CFB58DC980FAF76016C7 |
## Current Successful Gate Bindings

Read-only rehash confirms all **52** explicit final fleet source owners in `📥️fleet-final-source-bindings-2026-10-06.json` match. All **177** product-relative source hashes retained in `📓️fleet-api-layout-visual-2026-10-06.md` independently match current files. The four PDF digest rows were excluded from this source-path parser; no format-header or disappeared generated-PDF claim substitutes for byte provenance. Earlier final receipt and independent audit remain the actual four consumer-byte proofs; the cleanup purge removed this lane's generated publication artifacts.

All four exact strict60159 owners match current source:

| Owner relative to Print | Current SHA256 |
| --- | --- |
| `🎮️commands/🧪️print-pipeline-verification/🧪️tests/🖨️pipeline/🟦️.ts` | 2F5B32C5BD8B40D57F9C5830B4AF47DF33F8044DA45E3905E39FEB8C7B3CC4D9 |
| `🎮️commands/🧪️print-pipeline-verification/🟦️.ts` | 8978A28D5F7252772DB6B7419D8EF7E5D9ECEE9BB3BBF658789A31B19C34FC26 |
| `📦️packages/🟦️typescript/📜️script.ts` | C9F102E70B6D0A5236B6181330D92B697AFCFBC0968E0451FE248350C762F5EB |
| `🧬️schema/💡️inferences/📦️packages/🟦️typescript/🔬️probes/🟦️.ts` | 1D9408D3FCCDDA303B417503C995DBF5EE87B4CE808D69A44288C175A3129588 |

Actual64084 therefore remains current for its full API observer and bound publication inputs; actual60159 remains current for its five strict suites. Actual38092 remains accepted evidence for its own coverage/native-apparatus scope. This audit does not turn52/177 matching owners into an assertion that every transitive input of every historical task was captured.

## Requirement and Evidence Boundary

The ticket's original description requires a full D3-like **static** component library, documented applicable controls, neutral cases with independent third-party cross-validation, taxonomy completion and zero-touch commands. The authored architecture separates shared grammar/kernel algorithms from native family presets. Native presets intentionally can complete TiKZ while reporting a numerical scene unavailable; this is not a newly missing numerical-scene implementation. Existing multi-implementation admission/geometry gates,630 canonical checks, scoped Domain/Neural/Construction/Surface/Physics proofs and four-theme/language API proofs are meaningful evidence for their declared cases. They do not establish every possible option combination or an unexecuted operating system.

One concrete **remaining publication gate** exists beyond blocked generated-folder cleanup: the final current **81-owner/162-PDF catalogue consumer**. The latest delivery calls38092 a complete catalogue gate, but its registered target is `test-viz`, whose project command is `test viz coverage`. In the route, `verifyPrintVisualizationBuild()` runs only for `mode === "full"`. The coverage route observes8 carrier frames and30 paint controls, then structural1,966/1,738/API131 coverage; it does not read every catalogue PDF or perform the exact all-leaf painted census.

The existing `test-viz-full` target calls `test viz full`, depends on `build-viz`, and supplies the all-document independent PDF.js reader and exact1,966 leaves/1,738 kinds in each theme. The current source also retains equal-geometry groups for semantic/source adjudication; it does not automatically prove every kind's semantic distinctness or every control combination. This limitation is not a demonstrated new implementation defect.

Historical full13588 had actual exit0,81 pairs/162 reads,42m52s and60/87 cache hits. Its own retained receipt explicitly says it predates gallery intrinsic-height and planar effective-control changes. It also predates current Domain/Neural/Surface/Physics and authored API changes. Its reported aggregate is EB1F21A590C2737487AA7697C6EF812DD41326F54CFFD563A7EC9E6E014C33D0. The complete retained294-input table compared above has a different earlier aggregate b7a98daa…; it is not mislabeled as13588's later exact snapshot. That table alone shows122 changed paths:30 native library files,81 authored/generated documents and11 other compiler/control owners. Thus historical full publication cannot be transferred to current bytes. The already retained `📓️fleet-completion-crosscheck-2026-10-06.md` expressly requires complete81/162 scope, so this is completion of an existing gate rather than an invented broader test request.

## Existing Execution and Output Contract

Registered command: `bun nx run @semio-tech/print:test-viz-full --skip-nx-cache`, with ordinary bounded Nx parallelism (prior full wave used8). It requires no new target, permanent script or launch entry; generated launch already exposes `⚖️test-viz-full📓️print🟦️`.

Use current pinned Bun on PATH, NX_DAEMON=false, task-specific NX_WORKSPACE_DATA_DIRECTORY, SEMIO_TICKET_DIR, and private SEMIO_PRINT_OUTPUT_DIR/SEMIO_TEST_ARTIFACT_DIR. Compiler staging belongs under ticket `🗑️generated/print`; observer logs/census outputs belong under the caller's private generated directory. The canonical `buildRegisteredPrintTemplate` publishes through `printDocumentOutputDirectory`, which intentionally resolves **shared** `📦️packages/🟦️typescript/dist/documents/<id>`; SEMIO_PRINT_OUTPUT_DIR does not redirect those final PDFs. Existing atomic repository artifact publication owns replacements. Coordinate no concurrent publisher to those81 outputs. Fonts/toolchain/generation and81 inferred publishers are existing prerequisites. Historical42m52 is a reference duration, not a prediction for a source-current uncached run.

Root must reopen the ticket before this new validation. Core can own the registered execution after Root setup, capture fresh graph/compiler inputs and actual producer/consumer hashes, retain terminal counts and drift evidence, and investigate any actual failure. No full gate was launched by this read-only audit. No additional concrete missing implementation was established here; full native catalogue observation is required before such a verdict or completion claim.