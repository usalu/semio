# VDI Product Identity Audit

Read-only source review. No tests or builds launched.

Base for paths below: `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏭️vdi3805/🏅️standards/🔖️1/🪆️subsets/✳️any/`.

## Intended Identity

Article number is the documented native mutation identity. For rename-product, remove-product and change-product-configuration, each `🧬️schema/🧬️mutations/<kind>/🧬️schema/🔣️.json` defines id as String and describes it as “The catalogue product (article number) this mutation addresses.” Their UI reference kind is product. Mutation target descriptors return this payload id. AddProduct documentation explicitly names identity.article_number the format's native product key.

The canonical rows declaration in `🧬️schema/🔺️diff/🦀️.rs` defines Vdi3805ProductsRows key as row.identity.article_number.clone(). Thus changing only these mutation lookups to product.id would contradict existing keyed apply/inverse semantics.

Compliance owner `🧬️schema/💡️inferences/📚️document-checks/🗃️catalog/🗃️product-index/🪪️records/🦀️.rs` explicitly requires product.id == product.identity.article_number, plus record100 manufacturer/group/article consistency. Its check is vdi3805.1.identity.{article}. Different identities are represented but nonconforming, rather than documented independent identities.

## Exact Mutation Lookups

Within `🧬️schema/🧬️mutations/`:

| Kind | Diff lookup | Inverse lookup |
| --- | --- | --- |
| 🏷️rename-product | find identity.article_number == payload.id | same find; restores old title |
| 🎛️change-product-configuration | same find; sparse modification keyed by payload.id | same find; restores old configuration |
| 🗑️remove-product | position identity.article_number == payload.id | same position; restores product at original index |
| 📦️add-product | rejects duplicate identity.article_number | same duplicate check; inverse RemoveProduct.id = added article number |

The lookup implementations live in each kind's `🔺️diff/🦀️.rs` and `↩️inverse/🦀️.rs`. Missing target emits Error mutation.target-missing; add duplicate emits Fatal mutation.duplicate-id. These are coherent with the domain's article key.

## Generic Selector Boundary

`🧬️schema/🧬️mutations/🧭️edit-rules/🦀️.rs` binds products[].configuration and products[].title with SelectorKey::Id("id"), and collection removal with IdRemoval id:"id".

Shared `✏️s/🔌️plugins/📕️norm/📇️registry/🧬️contract/🖥️app-surface/🧭️value-path/🦀️.rs` extracts an object's literal id field for id selectors. Therefore a product with id="internal-X", article_number="article-Y" can be located by inspector path products[id=internal-X], while native mutation receives internal-X and refuses because its article key is article-Y. This is a confirmed source-level mismatch for an invalid-but-representable document, not for conforming examples.

Native SQLite persists/reconstructs both columns separately in `🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs`; JSON model exposes both independent Strings. The sync_typed_attributes_into_records helper in `🧬️schema/📸️snapshot/📚️catalog-model/🦀️.rs` fills id from article only if id is empty; it does not overwrite a nonempty differing id. Native text parser creates id from article number.

A bounded scan parsed172 committed JSON files beneath the VDI artifact. The only unequal product-like row found was malformed SQLite fixture `🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🧫️fixtures/🔣️.json`, id duplicate and empty article. No conforming committed example with unequal fields was found. Radiator RAD-600, pump PMP-50 and heat-generator HG-24 reference fixtures explicitly set both equal.

## Minimal Proposed Regression

Start conforming_blatt_3_dataset and assert both identities RAD-600. Run title/configuration edits and removal through canonical product-id paths and real typed application; this establishes the normal invariant case.

Separately clone it and set only product.id to internal-X. First assert the identity compliance check fails. Then show canonical path resolution can locate products[id=internal-X], while a directly constructed ChangeProductConfiguration{id:"internal-X",...} admits as Error mutation.target-missing and id:"RAD-600" reaches the native row. This documents invalid-document behavior without asserting a new identity policy.

Before changing production, choose a domain-owned policy for editing nonconforming identities: reject such rows explicitly at action resolution, or author a domain-owned selector-to-article extraction rule. Do not silently change native mutation identities or keyed diff laws based on this audit. No implementation change is justified for the existing conforming RAD-600 descendant witness.
