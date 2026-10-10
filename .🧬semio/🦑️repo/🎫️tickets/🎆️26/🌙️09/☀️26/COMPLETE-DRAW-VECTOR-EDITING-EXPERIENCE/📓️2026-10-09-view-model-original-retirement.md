
## Empty inspector fixture ownership check

The current ViewModel::new constructor (Manifest Rust line 7061) contains only caller-owned locale/terminology scalars, None optional fields, empty Vec::new lists, and default RetainedOrderedMap fields. The map default (ordered-map Rust line 18) is Vec::new with len zero. Borrowing TreeWindows::for_body does not transfer or allocate ViewModel ownership. Therefore an unchanged new ViewModel can leave scope without releasing physical heap storage. Any fixture that later populates strings, tree requests, window instances or maps must retire its original ViewModel with the actual ControlledRetirement facet. This source inspection is not a new allocator test. The parent inspector native invocation remains pending.

The latest portable owning-removal readiness replay completed successfully: three tests, 924 assertions, strict source exit zero. Native readiness additions have not been replayed separately.

The exact new Properties fixture was also inspected at selection Rust lines 457-459: its freshly constructed ViewModel remains unchanged through borrowed rendering, so the empty-owner conclusion applies directly. Older paged fixtures at lines 49,174,341 contain original populated tree_windows and remain a separate physical-retirement obligation when selected.

## Historical populated inspector fixtures repaired

The three populated tree-window fixtures now move their original ViewModel into retire_inspector_view after the last borrowed UI projection. The local test helper uses genuine ControlledRetirement<ViewModel>, retains the whole original on unexpected admission refusal, obtains each actual child copy/capacity/release/depth quote, and checks each independently caller-funded step against its grant. This preserves current parent eligibility changes and introduces no runtime API. Native selection verification remains pending behind the current Plugin/stdio compile frontier.
