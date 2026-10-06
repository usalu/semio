# Hub Fixture Relocation

Language-neutral examples moved to their semantic owners. Legacy fixture directory spellings, package-level fixture roots and redundant nested fixture wrappers were removed. The broker schema moved to schema ownership. The admin-intent oracle executable moved from fixtures to its canonical test case and the verification router now invokes that case. Each initial move preserved bytes; source/schema-path references were subsequently rebased. Runtime and complete reference verification are pending.

## Preserved Moves

```json
[
  {
    "old": "🌎️hub/🧪️fixtures/📇️directory/🔏️document-execution-target-lease-v1/🔣️.json",
    "new": "🌎️hub/📇️directory/🧫️fixtures/🔏️document-execution-target-lease-v1/🔣️.json",
    "sha256": "d7c99ec9ef696ffd7741d204f58dc279202fd393c4e46b846a5d7f76a0f783d5"
  },
  {
    "old": "🌎️hub/🧪️fixtures/📇️directory/🏘️space-administration-page-v1/🔣️.json",
    "new": "🌎️hub/📇️directory/🧫️fixtures/🏘️space-administration-page-v1/🔣️.json",
    "sha256": "eda274033c3bcb4e8fb1fd9a6a6990351c3afc84f334d857d0ca5b9f1a325d29"
  },
  {
    "old": "🌎️hub/🧪️fixtures/📇️directory/📅️event-page-route-v1/🔣️.json",
    "new": "🌎️hub/📇️directory/🧫️fixtures/📅️event-page-route-v1/🔣️.json",
    "sha256": "1e6556410f2cf66059adcb85b41ae5f222ea6a063afab5f20bbac1a248fadc54"
  },
  {
    "old": "🌎️hub/🧪️fixtures/📇️directory/🚻️space-journey-v1/🔣️.json",
    "new": "🌎️hub/📇️directory/🧫️fixtures/🚻️space-journey-v1/🔣️.json",
    "sha256": "3ab414029e240e66d10377cac892d32d4f23ea66fcf06c9f4488e0b9d8f110e5"
  },
  {
    "old": "🌎️hub/🧪️fixtures/📇️directory/🧾️command-receipt-v1/🔣️.json",
    "new": "🌎️hub/📇️directory/🧫️fixtures/🧾️command-receipt-v1/🔣️.json",
    "sha256": "50b4e36d05244c64f49cb306d086dad83e86189037da912629b852689e10498a"
  },
  {
    "old": "🌎️hub/🧪️fixtures/⛓️inference-wal-chain-v1/🔣️.json",
    "new": "🌎️hub/🧫️fixtures/⛓️inference-wal-chain-v1/🔣️.json",
    "sha256": "c7b1ff12b35c5ed4212dfffcd9fc0946f9f4bc20e5ab1682e5dfbe1e899f03db"
  },
  {
    "old": "🌎️hub/🧪️fixtures/⏸️gis-inference-checkpoint-control-v1/🔣️.json",
    "new": "🌎️hub/🧫️fixtures/⏸️gis-inference-checkpoint-control-v1/🔣️.json",
    "sha256": "99390b2c519697878998f21154eee2163f4a7fcdb256d145ca67b9abd87e1c0f"
  },
  {
    "old": "🌎️hub/🧪️fixtures/🧭️inference-job-reconcile-v1/🔣️.json",
    "new": "🌎️hub/🧫️fixtures/🧭️inference-job-reconcile-v1/🔣️.json",
    "sha256": "782ec74ec602acfd4e3b6ee551a8013c1c4317bb2c8d07591ab8462da838d2ec"
  },
  {
    "old": "🌎️hub/🧪️fixtures/🗺️gis-inference-job-v1/🔣️.json",
    "new": "🌎️hub/🧫️fixtures/🗺️gis-inference-job-v1/🔣️.json",
    "sha256": "d7347e8fde823000a2c20e079352cd1346c4d1817effa50d851dc0f325cb8554"
  },
  {
    "old": "🌎️hub/🧪️fixtures/🖥️inference-server-identity-v1/🔣️.json",
    "new": "🌎️hub/🧫️fixtures/🖥️inference-server-identity-v1/🔣️.json",
    "sha256": "9c05f4d00f030e9bab371a87fc63cd5417d69cb9ab8a76c0c6ca33e4d47fb94a"
  },
  {
    "old": "🌎️hub/🧪️fixtures/💡️inference-relay-v1/🔣️.json",
    "new": "🌎️hub/🧫️fixtures/💡️inference-relay-v1/🔣️.json",
    "sha256": "607ea5a2c6670095b9a3b44f8f439a4a442d86c62038441a900579b978529d3c"
  },
  {
    "old": "🌎️hub/🧪️fixtures/🪪️execution-target-relay-v1/🔣️.json",
    "new": "🌎️hub/🧫️fixtures/🪪️execution-target-relay-v1/🔣️.json",
    "sha256": "b5cd6a9403cbfc85fd54bbe6043b6403ec64d0a3935b6e568d436a9382db9871"
  },
  {
    "old": "🌎️hub/🧪️fixtures/🧊️gis-map-frozen-binding-v1/🔣️.json",
    "new": "🌎️hub/🧫️fixtures/🧊️gis-map-frozen-binding-v1/🔣️.json",
    "sha256": "193a413a985ad0ce94c70c5c97bf996c22c4777c43672f67d60102a7a9495d9a"
  },
  {
    "old": "🌎️hub/🧪️fixtures/🗳️gis-map-proposal-approval-v1/🔣️.json",
    "new": "🌎️hub/🧫️fixtures/🗳️gis-map-proposal-approval-v1/🔣️.json",
    "sha256": "1d8dccd5f766f50159a11041d656c01a47a04407f23ef9e9b388f9b9aba308f4"
  },
  {
    "old": "🌎️hub/🧪️fixtures/✅️inference-approval-v1/🔣️.json",
    "new": "🌎️hub/🧫️fixtures/✅️inference-approval-v1/🔣️.json",
    "sha256": "c5c8e860ed1966ae6e7f8ed521bcc24259ac62bfc38338aac106fc0e845b5eef"
  },
  {
    "old": "🌎️hub/🧪️fixtures/🎯️inference-catalog-selection-v1/🔣️.json",
    "new": "🌎️hub/🧫️fixtures/🎯️inference-catalog-selection-v1/🔣️.json",
    "sha256": "58917279c8c67a49ad7784a5db3ef07ee83b9bb7c2c86a8a9e8bc91de33932b1"
  },
  {
    "old": "🌎️hub/🧪️fixtures/✉️inference-command-v1/🔣️.json",
    "new": "🌎️hub/🧫️fixtures/✉️inference-command-v1/🔣️.json",
    "sha256": "f5927115a40b5f8bbeabaa204e707da0ba5cf034a17e8a3f9697eef72e4c2fa7"
  },
  {
    "old": "🌎️hub/🧪️fixtures/🧾️inference-wal-proof-v1/🔣️.json",
    "new": "🌎️hub/🧫️fixtures/🧾️inference-wal-proof-v1/🔣️.json",
    "sha256": "7f7a3be9adb6fe859ddcd1c6fb8ff7a917492b0a9b5ab7a5cd201465cc60e095"
  },
  {
    "old": "🌎️hub/🧪️fixtures/🤝️two-author-shell-v1/🔣️.json",
    "new": "🌎️hub/🧫️fixtures/🤝️two-author-shell-v1/🔣️.json",
    "sha256": "d28ed58b4413471ff59fb85be85b67c2627052b093d7c0702619d01015810b54"
  },
  {
    "old": "🌎️hub/🧪️fixtures/↩️gis-map-approval-undo-v1/🔣️.json",
    "new": "🌎️hub/🧫️fixtures/↩️gis-map-approval-undo-v1/🔣️.json",
    "sha256": "e567620aba482b24c3202797170898a0076ca60f847e9abfe8e209e69316ffbd"
  },
  {
    "old": "🌎️hub/🧪️fixtures/🔐️browser-broker-proof-lifecycle-v1/🔣️.json",
    "new": "🌎️hub/🧫️fixtures/🔐️browser-broker-proof-lifecycle-v1/🔣️.json",
    "sha256": "c316a333dc5a9634902ddad52bc39832ca046764213f6d317f49b18aecf91504"
  },
  {
    "old": "🌎️hub/🧪️fixtures/🔐️browser-broker-proof-lifecycle-v1/🧬️.schema.json",
    "new": "🌎️hub/🧬️schema/🔐️browser-broker-proof-lifecycle-v1/🔣️.json",
    "sha256": "d54b96485b1be501d9bd7388a4dda12122db670cd758222ef5d05ac055e80908"
  },
  {
    "old": "🌎️hub/🧪️fixtures/🛂️inference-author-v1/🔣️.json",
    "new": "🌎️hub/🧫️fixtures/🛂️inference-author-v1/🔣️.json",
    "sha256": "4f70743acb39e155dbb9d1147177da3ce44af1fcafa3d22b61631a9d14fb358d"
  },
  {
    "old": "🌎️hub/🧪️fixtures/🧭️native-artifact-provider-frontier-v1/🔣️.json",
    "new": "🌎️hub/🧫️fixtures/🧭️native-artifact-provider-frontier-v1/🔣️.json",
    "sha256": "9ef54b84190e2568d64453c25be5ba47608fb137e3314903d7811dc8d879bd79"
  },
  {
    "old": "🌎️hub/🧪️fixtures/🗺️gis-inference-retained-runtime-v1/🔣️.json",
    "new": "🌎️hub/🧫️fixtures/🗺️gis-inference-retained-runtime-v1/🔣️.json",
    "sha256": "81aa14cf6bf4792e8462b7feb9712f68fb83b3836dfbc3eaddfc52aace55a410"
  },
  {
    "old": "🌎️hub/📇️directory/🧫️fixtures/🧪️fixtures/📣️ordered-append-broadcast-v1/🔣️.json",
    "new": "🌎️hub/📇️directory/🧫️fixtures/📣️ordered-append-broadcast-v1/🔣️.json",
    "sha256": "0fac0296de1077f90975c4274a38688073b1d1a2204473c9b7bb26823b1a0f45"
  },
  {
    "old": "🌎️hub/📇️directory/🧫️fixtures/🧪️fixtures/🔌️scoped-socket-revocation-v1/🔣️.json",
    "new": "🌎️hub/📇️directory/🧫️fixtures/🔌️scoped-socket-revocation-v1/🔣️.json",
    "sha256": "a8172503f3ad98bbeb952516d869d14185b9e69f803134dc58afdb20ce967105"
  },
  {
    "old": "🌎️hub/🚀️local-bootstrap/🧪️fixtures/🚇️pipe-v1/🔣️.json",
    "new": "🌎️hub/🚀️local-bootstrap/🧫️fixtures/🚇️pipe-v1/🔣️.json",
    "sha256": "41dc591d8c0e0f673cc537a09d113f1bf3b715666c25ea4071ffeed89ee78ade"
  },
  {
    "old": "🌎️hub/🚀️local-bootstrap/🧪️fixtures/⏳️idle-admission-v1/🔣️.json",
    "new": "🌎️hub/🚀️local-bootstrap/🧫️fixtures/⏳️idle-admission-v1/🔣️.json",
    "sha256": "48b9d5e0e1dd1cdfbf129eb80955456a95fa55c92e695081fc11914425669344"
  },
  {
    "old": "🌎️hub/🔐️auth/🧪️fixtures/🔑️capability-v1/🔣️.json",
    "new": "🌎️hub/🔐️auth/🧫️fixtures/🔑️capability-v1/🔣️.json",
    "sha256": "d1b01d5097fe34ddd025ed975b069dcb0e61507ccad82be1b0157a56912c1983"
  },
  {
    "old": "🌎️hub/🗿️artifact-authority/🧪️fixtures/🏛️canonical-authority/🔣️.json",
    "new": "🌎️hub/🗿️artifact-authority/🧫️fixtures/🏛️canonical-authority/🔣️.json",
    "sha256": "48bdf8a42f290192b81276ab8f5ad4672225b0ba679e6301d17c1352534ad98b"
  },
  {
    "old": "🌎️hub/🗿️artifact-authority/🧪️fixtures/🔌️authority-adapter/🔣️.json",
    "new": "🌎️hub/🗿️artifact-authority/🧫️fixtures/🔌️authority-adapter/🔣️.json",
    "sha256": "2e5031f6f1ced8384677bfa042af813e1d15976c0fd36ae4e8af5d4288458f98"
  },
  {
    "old": "🌎️hub/🗿️artifact-authority/🧪️fixtures/🧱️artifact-chunk-cas/🔣️.json",
    "new": "🌎️hub/🗿️artifact-authority/🧫️fixtures/🧱️artifact-chunk-cas/🔣️.json",
    "sha256": "5f4fe6d8c0a17e54d147c3279f415b17364ffc6562eb379840dc3c3db2ac75ae"
  },
  {
    "old": "🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🧪️fixtures/🛡️opened-root/🔣️.json",
    "new": "🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🧫️fixtures/🛡️opened-root/🔣️.json",
    "sha256": "57a2ea1c387bc0b95bf40626b6811cbac9ac53d58445a40f0a57d76c2eb4f0a1"
  },
  {
    "old": "🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🧪️fixtures/🧊️codec-source/🔣️.json",
    "new": "🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🧫️fixtures/🧊️codec-source/🔣️.json",
    "sha256": "e7f4d1727d28e7dad3c206d5be7ff10b246f4c263233c3e3df766340bf5cc052"
  },
  {
    "old": "🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🧪️fixtures/🧬️retained-gis-browser/🔣️.json",
    "new": "🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🧫️fixtures/🧬️retained-gis-browser/🔣️.json",
    "sha256": "19c4463bbb3b17f01983d7d2b953c644c28a79dfe7e1ef73264ab44cc09c4f05"
  },
  {
    "old": "🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🧪️fixtures/🧱️generation-stage/🔣️.json",
    "new": "🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🧫️fixtures/🧱️generation-stage/🔣️.json",
    "sha256": "e7bf5272697e9e4ce081aca7fa66558675db71ed1aa62f710130331d750a3272"
  },
  {
    "old": "🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🧪️fixtures/🧬️stdio-gis-bootstrap/🔣️.json",
    "new": "🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🧫️fixtures/🧬️stdio-gis-bootstrap/🔣️.json",
    "sha256": "ab669d3d812b816bcc5af9a9c26d05deb2fb5c190bd57765392cb2af9b721fc8"
  },
  {
    "old": "🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🧪️fixtures/📤️publication/🔣️.json",
    "new": "🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🧫️fixtures/📤️publication/🔣️.json",
    "sha256": "38de0f38fdccf0784cd813f6bcf8fb158e8b84c3c155332efad7d82fc241e6b5"
  },
  {
    "old": "🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🧪️fixtures/📤️publication/🔒️owner.json",
    "new": "🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🧫️fixtures/📤️publication/🔒️owner.json",
    "sha256": "608b55e164199760e95b84d995ef94f26e0158ec32d263bb952a9010e71e0907"
  },
  {
    "old": "🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🧪️fixtures/📤️publication/📬️command.json",
    "new": "🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🧫️fixtures/📤️publication/📬️command.json",
    "sha256": "a7a25aa2e907432cb2d0234ed3ba083bdf88407c18d2184b054b0102fd94fb9a"
  },
  {
    "old": "🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🧪️fixtures/📤️publication/🔄️cas.json",
    "new": "🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🧫️fixtures/📤️publication/🔄️cas.json",
    "sha256": "1b558a3f88e1163e059ec783cc16ae6f086028c75e8f324d00be7bd122d5ec99"
  },
  {
    "old": "🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🧪️fixtures/📤️publication/📡️transport.json",
    "new": "🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🧫️fixtures/📤️publication/📡️transport.json",
    "sha256": "d4238f58da77dc9115a27ffd70e818e3d528904b7e34fc206cb544b2b8db1f46"
  },
  {
    "old": "🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🧪️fixtures/👥️two-package/🔣️.json",
    "new": "🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🧫️fixtures/👥️two-package/🔣️.json",
    "sha256": "0db9bc71321bbc4810b8ba4b6c34df6ed1cebe0187a349b500444e6bee850f95"
  },
  {
    "old": "🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🧪️fixtures/🪪️identity-roles/🔣️.json",
    "new": "🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🧫️fixtures/🪪️identity-roles/🔣️.json",
    "sha256": "6f393337416eb5a34ceeed2a6cbe22a365536a9c9d30306a60f1e8406ceaca03"
  },
  {
    "old": "🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🧪️fixtures/🌐️browser-actor/🔣️.json",
    "new": "🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🧫️fixtures/🌐️browser-actor/🔣️.json",
    "sha256": "d5a621efb4cc94b400a7bb976a0771332175c29baeaaba84d5af9f9b40d8a73d"
  },
  {
    "old": "🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🧪️fixtures/🤝️gis-map-collaboration/🔣️.json",
    "new": "🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🧫️fixtures/🤝️gis-map-collaboration/🔣️.json",
    "sha256": "f64abad1f0650326accd1e45c9126ed528e5c5155d83c3305eb6e46132b209f4"
  },
  {
    "old": "🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🧪️fixtures/🔗️compiled-dependencies/🔣️.json",
    "new": "🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🧫️fixtures/🔗️compiled-dependencies/🔣️.json",
    "sha256": "2a39e87dee238a84e0e79bd9ce7409ea45526cd93a292768a8ec2d9e2b1ff52a"
  },
  {
    "old": "🌎️hub/🗿️artifact-authority/📇️native-openable-provider/🧪️fixtures/🪪️v1/🔣️.json",
    "new": "🌎️hub/🗿️artifact-authority/📇️native-openable-provider/🧫️fixtures/🪪️v1/🔣️.json",
    "sha256": "a3277a2480af58c93ac01eab9c3195224749dcc3500becc0ffba024aa9277504"
  },
  {
    "old": "🌎️hub/🗿️artifact-authority/📇️native-openable-provider/🧪️fixtures/🌍️gis-v1/🔣️.json",
    "new": "🌎️hub/🗿️artifact-authority/📇️native-openable-provider/🧫️fixtures/🌍️gis-v1/🔣️.json",
    "sha256": "ab5e29af2bb00441f83aa7b156354e6c6dd7259d7a47eb580b54e0ccd9ca04a4"
  },
  {
    "old": "🌎️hub/🗿️artifact-authority/📇️native-openable-provider/🧪️fixtures/🌿️vcs-v1/🔣️.json",
    "new": "🌎️hub/🗿️artifact-authority/📇️native-openable-provider/🧫️fixtures/🌿️vcs-v1/🔣️.json",
    "sha256": "8403c6a5469722f960e40f38ba88c6473ff2411c4fec62dfbf0a0e70d992c39d"
  },
  {
    "old": "🌎️hub/🛰️lag-rebootstrap/🧪️fixtures/🪢️canonical-pair/🔣️.json",
    "new": "🌎️hub/🛰️lag-rebootstrap/🧫️fixtures/🪢️canonical-pair/🔣️.json",
    "sha256": "5dec0e22bd0458253df8f6460924adeb57b79964ff7914ae331f42a8f4dc6037"
  },
  {
    "old": "🌎️hub/🛰️lag-rebootstrap/🧪️fixtures/🛟️lag-rebootstrap/🔣️.json",
    "new": "🌎️hub/🛰️lag-rebootstrap/🧫️fixtures/🛟️lag-rebootstrap/🔣️.json",
    "sha256": "71ab341d8a14ad32639c8b134b72ba8d3e2bb362a41d03914f846a4defb02f1f"
  },
  {
    "old": "🌎️hub/📦️packages/🦀️rust/🧪️fixtures/🪪️presence-normalization-v1/🧪️fixture/🔣️.json",
    "new": "🌎️hub/🧫️fixtures/🪪️presence-normalization-v1/🔣️.json",
    "sha256": "7615c2572c8724ae28b643627b84d101a7e73a40f75e6f345990da4e9f0609ad"
  },
  {
    "old": "🌎️hub/📦️packages/🦀️rust/🧪️fixtures/👥️presence-lease-v1/🧪️fixture/🔣️.json",
    "new": "🌎️hub/🧫️fixtures/👥️presence-lease-v1/🔣️.json",
    "sha256": "df91271aa330ce9e25ce96b63cfaefdb890856d53fe9b52deed606ee76fef8ac"
  },
  {
    "old": "🌎️hub/📦️packages/🦀️rust/🧪️fixtures/🌐️directory-message-authority-v1/🔣️.json",
    "new": "🌎️hub/🧫️fixtures/🌐️directory-message-authority-v1/🔣️.json",
    "sha256": "216ce939999c93b3b2d8fa86bb80d07df99ed9e478ca83bcede7e66683a089c7"
  },
  {
    "old": "🌎️hub/📦️packages/🦀️rust/🧪️fixtures/🛂️admin-presence-target-recovery-v1/🔣️.json",
    "new": "🌎️hub/🧫️fixtures/🛂️admin-presence-target-recovery-v1/🔣️.json",
    "sha256": "05e2913cf0f523998fbcf9de5ae4f0879d142d4ac38f5d2c5412972660abd8aa"
  },
  {
    "old": "🌎️hub/📦️packages/🦀️rust/🧪️fixtures/🚧️hub-boundaries/🔣️.json",
    "new": "🌎️hub/🧫️fixtures/🚧️hub-boundaries/🔣️.json",
    "sha256": "a7552b66363cc873bf785020d8296d79077abb46a6c8a3b6a039bfe08df7a326"
  },
  {
    "old": "🌎️hub/📦️packages/🦀️rust/🧪️fixtures/🏛️admin-directory-authority-v1/🔣️.json",
    "new": "🌎️hub/🧫️fixtures/🏛️admin-directory-authority-v1/🔣️.json",
    "sha256": "4dbe11007277a0d0cccfb64c04db998aea0e204f25095b6e92a06d845bfc46f0"
  },
  {
    "old": "🌎️hub/📇️directory/🧪️tests/📸️artifact-checkpoint-projection.json",
    "new": "🌎️hub/📇️directory/🧫️fixtures/📸️artifact-checkpoint-projection/🔣️.json",
    "sha256": "68f0b27105c195ae70143bac7e8c13daa35ee702a20a2b89f144039c9051ea14"
  },
  {
    "old": "🌎️hub/📇️directory/🧪️tests/🔑️share-token-vectors.json",
    "new": "🌎️hub/📇️directory/🧫️fixtures/🔑️share-token-vectors/🔣️.json",
    "sha256": "86321b226e374ac12dd6a77de674974c33c6fd74f2a14fd7e7d5cc3231f54b8f"
  },
  {
    "old": "🌎️hub/📇️directory/🧪️tests/🏛️retained-short-admin/🔣️.json",
    "new": "🌎️hub/📇️directory/🧫️fixtures/🏛️retained-short-admin/🔣️.json",
    "sha256": "4e16f6b088132dbb5ad3f9010d3d6a1c4c9b2197177dfe7a59ce6449cb7f802e"
  },
  {
    "old": "🌎️hub/📇️directory/🧪️tests/🔐️share-issuance-atomicity/🔣️.json",
    "new": "🌎️hub/📇️directory/🧫️fixtures/🔐️share-issuance-atomicity/🔣️.json",
    "sha256": "6e2b4e3ccf7b54983f7f6ddf4f1d33cb116f90aa524dc856f8896fcfce8c5a5e"
  },
  {
    "old": "🌎️hub/📇️directory/🧫️fixtures/🎯️admin-intent-v1/🧪️oracle/🟦️.ts",
    "new": "🌎️hub/📇️directory/🧪️tests/🎯️admin-intent-v1/🟦️.ts",
    "sha256": "4fa40c5ba51caeba8a099e5694cdfe0596d9b14f387b729a11d8ddbfc9ef128c"
  }
]
```

## Authored Paths

```json
[
  ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️05/☀️30/ASSETS-FIXTURES-SEPARATION/📓️hub-fixture-relocation-2026-09-09.md",
  "✏️s/🔌️plugins/🗄️stdio/🧪️tests/📇️native-openable-provider/🦀️.rs",
  "🌎️hub/💡️inference/✉️command/🧪️tests/🔬️unit/🦀️.rs",
  "🌎️hub/💡️inference/🏃️runtime/🧪️tests/🔬️unit/🦀️.rs",
  "🌎️hub/💡️inference/📇️catalog/🧪️tests/🔬️unit/🦀️.rs",
  "🌎️hub/💡️inference/🛂️authorization/🧪️tests/🔬️unit/🦀️.rs",
  "🌎️hub/💡️inference/🧬️schema/✅️approval/🧪️tests/🔬️unit/🦀️.rs",
  "🌎️hub/💡️inference/🧬️schema/🧪️tests/🔬️unit/🦀️.rs",
  "🌎️hub/💡️inference/🧾️wal/🧪️tests/⛓️chain/🦀️.rs",
  "🌎️hub/💡️inference/🧾️wal/🧪️tests/🔬️unit/🦀️.rs",
  "🌎️hub/💡️inference/🪶️sqlite/🧪️tests/🔬️unit/🦀️.rs",
  "🌎️hub/📇️directory/🧪️tests/🎯️admin-intent-v1/🟦️.ts",
  "🌎️hub/📇️directory/🧪️tests/🏛️retained-short-admin/🔣️.json",
  "🌎️hub/📇️directory/🧪️tests/📸️artifact-checkpoint-projection.json",
  "🌎️hub/📇️directory/🧪️tests/🔐️share-issuance-atomicity/🔣️.json",
  "🌎️hub/📇️directory/🧪️tests/🔑️share-token-vectors.json",
  "🌎️hub/📇️directory/🧪️tests/🔬️unit/🦀️.rs",
  "🌎️hub/📇️directory/🧫️fixtures/🎯️admin-intent-v1/🧪️oracle/🟦️.ts",
  "🌎️hub/📇️directory/🧫️fixtures/🏘️space-administration-page-v1/🔣️.json",
  "🌎️hub/📇️directory/🧫️fixtures/🏛️retained-short-admin/🔣️.json",
  "🌎️hub/📇️directory/🧫️fixtures/📅️event-page-route-v1/🔣️.json",
  "🌎️hub/📇️directory/🧫️fixtures/📣️ordered-append-broadcast-v1/🔣️.json",
  "🌎️hub/📇️directory/🧫️fixtures/📸️artifact-checkpoint-projection/🔣️.json",
  "🌎️hub/📇️directory/🧫️fixtures/🔌️scoped-socket-revocation-v1/🔣️.json",
  "🌎️hub/📇️directory/🧫️fixtures/🔏️document-execution-target-lease-v1/🔣️.json",
  "🌎️hub/📇️directory/🧫️fixtures/🔐️share-issuance-atomicity/🔣️.json",
  "🌎️hub/📇️directory/🧫️fixtures/🔑️share-token-vectors/🔣️.json",
  "🌎️hub/📇️directory/🧫️fixtures/🚻️space-journey-v1/🔣️.json",
  "🌎️hub/📇️directory/🧫️fixtures/🧪️fixtures/📣️ordered-append-broadcast-v1/🔣️.json",
  "🌎️hub/📇️directory/🧫️fixtures/🧪️fixtures/🔌️scoped-socket-revocation-v1/🔣️.json",
  "🌎️hub/📇️directory/🧫️fixtures/🧾️command-receipt-v1/🔣️.json",
  "🌎️hub/📇️directory/🪶️sqlite/🧪️tests/🔬️unit/🦀️.rs",
  "🌎️hub/📦️packages/🦀️rust/📜️script.ts",
  "🌎️hub/📦️packages/🦀️rust/🧪️fixtures/🌐️directory-message-authority-v1/🔣️.json",
  "🌎️hub/📦️packages/🦀️rust/🧪️fixtures/🏛️admin-directory-authority-v1/🔣️.json",
  "🌎️hub/📦️packages/🦀️rust/🧪️fixtures/👥️presence-lease-v1/🧪️fixture/🔣️.json",
  "🌎️hub/📦️packages/🦀️rust/🧪️fixtures/🚧️hub-boundaries/🔣️.json",
  "🌎️hub/📦️packages/🦀️rust/🧪️fixtures/🛂️admin-presence-target-recovery-v1/🔣️.json",
  "🌎️hub/📦️packages/🦀️rust/🧪️fixtures/🪪️presence-normalization-v1/🧪️fixture/🔣️.json",
  "🌎️hub/🔐️auth/🧪️fixtures/🔑️capability-v1/🔣️.json",
  "🌎️hub/🔐️auth/🧫️fixtures/🔑️capability-v1/🔣️.json",
  "🌎️hub/🗿️artifact-authority/📇️native-openable-provider/🧪️fixtures/🌍️gis-v1/🔣️.json",
  "🌎️hub/🗿️artifact-authority/📇️native-openable-provider/🧪️fixtures/🌿️vcs-v1/🔣️.json",
  "🌎️hub/🗿️artifact-authority/📇️native-openable-provider/🧪️fixtures/🪪️v1/🔣️.json",
  "🌎️hub/🗿️artifact-authority/📇️native-openable-provider/🧪️tests/🔬️unit/🦀️.rs",
  "🌎️hub/🗿️artifact-authority/📇️native-openable-provider/🧫️fixtures/🌍️gis-v1/🔣️.json",
  "🌎️hub/🗿️artifact-authority/📇️native-openable-provider/🧫️fixtures/🌿️vcs-v1/🔣️.json",
  "🌎️hub/🗿️artifact-authority/📇️native-openable-provider/🧫️fixtures/🪪️v1/🔣️.json",
  "🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🌐️browser-actor/🧪️tests/🔬️unit/🦀️.rs",
  "🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/📤️command/🧪️tests/🔬️unit/🦀️.rs",
  "🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🛡️opened-root/🧪️tests/🔬️publication/🦀️.rs",
  "🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🧪️fixtures/🌐️browser-actor/🔣️.json",
  "🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🧪️fixtures/👥️two-package/🔣️.json",
  "🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🧪️fixtures/📤️publication/📡️transport.json",
  "🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🧪️fixtures/📤️publication/📬️command.json",
  "🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🧪️fixtures/📤️publication/🔄️cas.json",
  "🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🧪️fixtures/📤️publication/🔒️owner.json",
  "🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🧪️fixtures/📤️publication/🔣️.json",
  "🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🧪️fixtures/🔗️compiled-dependencies/🔣️.json",
  "🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🧪️fixtures/🛡️opened-root/🔣️.json",
  "🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🧪️fixtures/🤝️gis-map-collaboration/🔣️.json",
  "🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🧪️fixtures/🧊️codec-source/🔣️.json",
  "🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🧪️fixtures/🧬️retained-gis-browser/🔣️.json",
  "🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🧪️fixtures/🧬️stdio-gis-bootstrap/🔣️.json",
  "🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🧪️fixtures/🧱️generation-stage/🔣️.json",
  "🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🧪️fixtures/🪪️identity-roles/🔣️.json",
  "🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🧪️tests/📤️publication/🦀️.rs",
  "🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🧪️tests/🔬️unit/🦀️.rs",
  "🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🧫️fixtures/🌐️browser-actor/🔣️.json",
  "🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🧫️fixtures/👥️two-package/🔣️.json",
  "🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🧫️fixtures/📤️publication/📡️transport.json",
  "🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🧫️fixtures/📤️publication/📬️command.json",
  "🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🧫️fixtures/📤️publication/🔄️cas.json",
  "🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🧫️fixtures/📤️publication/🔒️owner.json",
  "🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🧫️fixtures/📤️publication/🔣️.json",
  "🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🧫️fixtures/🔗️compiled-dependencies/🔣️.json",
  "🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🧫️fixtures/🛡️opened-root/🔣️.json",
  "🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🧫️fixtures/🤝️gis-map-collaboration/🔣️.json",
  "🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🧫️fixtures/🧊️codec-source/🔣️.json",
  "🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🧫️fixtures/🧬️retained-gis-browser/🔣️.json",
  "🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🧫️fixtures/🧬️stdio-gis-bootstrap/🔣️.json",
  "🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🧫️fixtures/🧱️generation-stage/🔣️.json",
  "🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🧫️fixtures/🪪️identity-roles/🔣️.json",
  "🌎️hub/🗿️artifact-authority/🧪️fixtures/🏛️canonical-authority/🔣️.json",
  "🌎️hub/🗿️artifact-authority/🧪️fixtures/🔌️authority-adapter/🔣️.json",
  "🌎️hub/🗿️artifact-authority/🧪️fixtures/🧱️artifact-chunk-cas/🔣️.json",
  "🌎️hub/🗿️artifact-authority/🧪️tests/🔬️unit/🦀️.rs",
  "🌎️hub/🗿️artifact-authority/🧫️fixtures/🏛️canonical-authority/🔣️.json",
  "🌎️hub/🗿️artifact-authority/🧫️fixtures/🔌️authority-adapter/🔣️.json",
  "🌎️hub/🗿️artifact-authority/🧫️fixtures/🧱️artifact-chunk-cas/🔣️.json",
  "🌎️hub/🗿️artifact-authority/🧱️chunk-cas/🧪️tests/🔬️unit/🦀️.rs",
  "🌎️hub/🚀️local-bootstrap/🧪️fixtures/⏳️idle-admission-v1/🔣️.json",
  "🌎️hub/🚀️local-bootstrap/🧪️fixtures/🚇️pipe-v1/🔣️.json",
  "🌎️hub/🚀️local-bootstrap/🧪️tests/🔬️unit/🦀️.rs",
  "🌎️hub/🚀️local-bootstrap/🧫️fixtures/⏳️idle-admission-v1/🔣️.json",
  "🌎️hub/🚀️local-bootstrap/🧫️fixtures/🚇️pipe-v1/🔣️.json",
  "🌎️hub/🛰️lag-rebootstrap/🧪️fixtures/🛟️lag-rebootstrap/🔣️.json",
  "🌎️hub/🛰️lag-rebootstrap/🧪️fixtures/🪢️canonical-pair/🔣️.json",
  "🌎️hub/🛰️lag-rebootstrap/🧪️tests/🔬️unit/🦀️.rs",
  "🌎️hub/🛰️lag-rebootstrap/🧫️fixtures/🛟️lag-rebootstrap/🔣️.json",
  "🌎️hub/🛰️lag-rebootstrap/🧫️fixtures/🪢️canonical-pair/🔣️.json",
  "🌎️hub/🧪️fixtures/↩️gis-map-approval-undo-v1/🔣️.json",
  "🌎️hub/🧪️fixtures/⏸️gis-inference-checkpoint-control-v1/🔣️.json",
  "🌎️hub/🧪️fixtures/⛓️inference-wal-chain-v1/🔣️.json",
  "🌎️hub/🧪️fixtures/✅️inference-approval-v1/🔣️.json",
  "🌎️hub/🧪️fixtures/✉️inference-command-v1/🔣️.json",
  "🌎️hub/🧪️fixtures/🎯️inference-catalog-selection-v1/🔣️.json",
  "🌎️hub/🧪️fixtures/💡️inference-relay-v1/🔣️.json",
  "🌎️hub/🧪️fixtures/📇️directory/🏘️space-administration-page-v1/🔣️.json",
  "🌎️hub/🧪️fixtures/📇️directory/📅️event-page-route-v1/🔣️.json",
  "🌎️hub/🧪️fixtures/📇️directory/🔏️document-execution-target-lease-v1/🔣️.json",
  "🌎️hub/🧪️fixtures/📇️directory/🚻️space-journey-v1/🔣️.json",
  "🌎️hub/🧪️fixtures/📇️directory/🧾️command-receipt-v1/🔣️.json",
  "🌎️hub/🧪️fixtures/🔐️browser-broker-proof-lifecycle-v1/🔣️.json",
  "🌎️hub/🧪️fixtures/🔐️browser-broker-proof-lifecycle-v1/🧬️.schema.json",
  "🌎️hub/🧪️fixtures/🖥️inference-server-identity-v1/🔣️.json",
  "🌎️hub/🧪️fixtures/🗳️gis-map-proposal-approval-v1/🔣️.json",
  "🌎️hub/🧪️fixtures/🗺️gis-inference-job-v1/🔣️.json",
  "🌎️hub/🧪️fixtures/🗺️gis-inference-retained-runtime-v1/🔣️.json",
  "🌎️hub/🧪️fixtures/🛂️inference-author-v1/🔣️.json",
  "🌎️hub/🧪️fixtures/🤝️two-author-shell-v1/🔣️.json",
  "🌎️hub/🧪️fixtures/🧊️gis-map-frozen-binding-v1/🔣️.json",
  "🌎️hub/🧪️fixtures/🧭️inference-job-reconcile-v1/🔣️.json",
  "🌎️hub/🧪️fixtures/🧭️native-artifact-provider-frontier-v1/🔣️.json",
  "🌎️hub/🧪️fixtures/🧾️inference-wal-proof-v1/🔣️.json",
  "🌎️hub/🧪️fixtures/🪪️execution-target-relay-v1/🔣️.json",
  "🌎️hub/🧪️tests/🔬️bin-unit/🦀️.rs",
  "🌎️hub/🧪️tests/🔬️standalone/🦀️.rs",
  "🌎️hub/🧪️tests/🤝️integration/🟦️.ts",
  "🌎️hub/🧫️fixtures/↩️gis-map-approval-undo-v1/🔣️.json",
  "🌎️hub/🧫️fixtures/⏸️gis-inference-checkpoint-control-v1/🔣️.json",
  "🌎️hub/🧫️fixtures/⛓️inference-wal-chain-v1/🔣️.json",
  "🌎️hub/🧫️fixtures/✅️inference-approval-v1/🔣️.json",
  "🌎️hub/🧫️fixtures/✉️inference-command-v1/🔣️.json",
  "🌎️hub/🧫️fixtures/🌐️directory-message-authority-v1/🔣️.json",
  "🌎️hub/🧫️fixtures/🎯️inference-catalog-selection-v1/🔣️.json",
  "🌎️hub/🧫️fixtures/🏛️admin-directory-authority-v1/🔣️.json",
  "🌎️hub/🧫️fixtures/👥️presence-lease-v1/🔣️.json",
  "🌎️hub/🧫️fixtures/💡️inference-relay-v1/🔣️.json",
  "🌎️hub/🧫️fixtures/🔐️browser-broker-proof-lifecycle-v1/🔣️.json",
  "🌎️hub/🧫️fixtures/🖥️inference-server-identity-v1/🔣️.json",
  "🌎️hub/🧫️fixtures/🗳️gis-map-proposal-approval-v1/🔣️.json",
  "🌎️hub/🧫️fixtures/🗺️gis-inference-job-v1/🔣️.json",
  "🌎️hub/🧫️fixtures/🗺️gis-inference-retained-runtime-v1/🔣️.json",
  "🌎️hub/🧫️fixtures/🚧️hub-boundaries/🔣️.json",
  "🌎️hub/🧫️fixtures/🛂️admin-presence-target-recovery-v1/🔣️.json",
  "🌎️hub/🧫️fixtures/🛂️inference-author-v1/🔣️.json",
  "🌎️hub/🧫️fixtures/🤝️two-author-shell-v1/🔣️.json",
  "🌎️hub/🧫️fixtures/🧊️gis-map-frozen-binding-v1/🔣️.json",
  "🌎️hub/🧫️fixtures/🧭️inference-job-reconcile-v1/🔣️.json",
  "🌎️hub/🧫️fixtures/🧭️native-artifact-provider-frontier-v1/🔣️.json",
  "🌎️hub/🧫️fixtures/🧾️inference-wal-proof-v1/🔣️.json",
  "🌎️hub/🧫️fixtures/🪪️execution-target-relay-v1/🔣️.json",
  "🌎️hub/🧫️fixtures/🪪️presence-normalization-v1/🔣️.json",
  "🌎️hub/🧬️schema/🔐️browser-broker-proof-lifecycle-v1/🔣️.json",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🏠️workspace/🔗️remote/🧪️tests/🔬️unit/🦀️.rs",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🏠️workspace/🧪️tests/🔬️quick/🦀️.rs",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/💡️inference/🧪️tests/🔬️inference-jobs/🦀️.rs",
  "🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🔌️client/🧪️tests/🔬️unit/🦀️.rs",
  "🧰️framework/🛍️products/💻️os/🧪️tests/🧪️space-artifact-creation-owner/🟦️.ts"
]
```

## Cross-Owner Consumers

9 live exact-path consumers now reference the canonical Hub destinations. Their paths are retained above; the earlier broad pass was interrupted by a concurrently moved plugin snapshot, then this bounded pass re-read the current consumers. Relative include validation remains separate.

## Admin Intent Runtime Verification

The relocated canonical TypeScript admin-intent oracle executed successfully through the public Bun/Nx route: 5/5 oracle checks, 22/22 invalid inventory, and 5/5 owned hub.directory schema exports. The actual process exited zero. The temporary log is `coordinator/hub-admin-intent.log`. This validates the relocated oracle and its fixture/schema reads; it does not claim a complete Hub Rust build.
