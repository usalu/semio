# Roof Reserve Window Overlap R95

The full native-polyhedral roof run settled RED after 810.348 s. It had cleared all paving reserves and materialized 18,067 exact gravel cells with real host holes, column/bore exclusions, and the declared guard interface. It then rejected an exact positive overlap between declared reserve windows. The attempted own cooperative cancellation occurred after the process had already settled; no peer process was controlled. This is partial correctness/performance evidence, not complete roof acceptance.

A separate bounded exact 2D diagnostic isolated the first overlapping authored windows in 1.425 s:

- Current: RES-FIN-ROOF-GRAVEL-SL-R-151, owner FIN-ROOF-GRAVEL.
- Earlier: RES-SALON-CHIMNEY-FINISH, owner CHIMNEY-SALON-ROOF.
- The gravel window uses X=12.835 while the actual chimney shell/window uses X=12.834999999999999.
- Exact intersection area is `178359884269560129097185041875 / 10040768854177940974038103865063019527905738752` m², approximately 1.7763568394002517e-17 m².

The diagnostic concerns control windows; it does not by itself prove current occupied-body overlap. The generic physical repair must preserve the chimney's dimensions and retain the gravel's original Reference. An explicitly declared material interface should trim finish material against the actual independent masonry body. No coordinate rounding or positive-area threshold should hide the witness.

Reserve windows are ownership queries rather than independent physical solids. Their expected stock must be decomposed by actual already assigned material, with exact union coverage and strict exclusion between distinct physical owners. A repeated physical owner may appear in more than one overlapping window without counting its material twice. Overlap between two different actual owned materials must remain an error. This contract needs new language-neutral controls before implementation, including identical/partial repeated windows, a true 2 nm cross-owner overlap, a 2 nm gap, and overlap outside occupied host stock.

The forthcoming semantic CSG path is important for memory and runtime: repeated finite circular cuts expanded a single 1,793-part source finish into 18,067 exact cells. Root reported one observed job near 6.5 GB and requested an own 5.5 GiB bound. Subsequent owned proof runners must record their own PID/RSS, preserve terminal/cancellation status, and stop cooperatively at the bound. This report does not attribute unrelated host jobs or authorize controlling them.

Six schema-first window controls were observed RED, then GREEN in 5.838 s: duplicate and partial same-owner windows, overlapping windows outside actual host material, an explicit masonry displacement, a true 2 nm cross-owner occupied overlap, and a true 2 nm missing-material gap. Windows now query actual material union. Previously assigned material is retained once, while a new actual owner is checked against the existing material before any query subtraction; overlapping windows cannot hide a physical collision. Structural-only wall/filling validation remains separate from generic declared material interfaces for finish receivers.

The exact masonry-window inventory found seventeen contacts, all the same FIN-ROOF-GRAVEL / CHIMNEY-SALON-ROOF pair; no additional masonry product was inferred. A bounded two-window physical probe with this explicit displacement passed in 19.175 s, with own PID 7456 and sampled peak 333,246,464 bytes. It reused `fixture-observer-r95.py`, hash 661ce9c61822d9179106a9e5dbd9f220bf64c8f8681b186b9820b13f86d4ae08. The observer controls only its own process.

The one-object declaration was published after strict schema validation: source 7f90ee18f0f9ef989083c4c1247d8fa40aa725a2b2dffd380cde7ca3a7eeb978 became f6116f9921b6d0b6a7bd236604c34a8d12acd1e6ead884eb9d41b542faecca2c. Only FIN-ROOF-GRAVEL's explicit interface list and qualified bilingual material interpretation changed. Original source parts, dimensions, GUIDs, all other objects and root fixture tags remain unchanged.

The bound-layer recipe also needs one physical union for repeated same-product control windows. A new independent GEOS/SciPy fixture observed the previous duplicate-volume RED. Its exact disjoint decomposition and reuse of all declared joint owners are implemented and awaiting the full portable layer rerun. Complete semantic roof, native SDK, and all-product interference acceptance remain pending.
