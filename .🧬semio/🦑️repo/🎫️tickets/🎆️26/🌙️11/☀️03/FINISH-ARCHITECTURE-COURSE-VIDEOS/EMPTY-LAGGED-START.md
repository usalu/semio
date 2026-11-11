# Empty Lagged Start

`Beat4_PartialModels` died during the architecture class succession at frame 102 of 161.

`ValueError: Trying to play LaggedStart(Group) without animations` came from `_lit` when a class had tags and none of those tags had a drawn part. The same empty `LaggedStart` was possible for an empty owner-tag list and an empty member-part list.

Those three paths now play `Wait(0.01)` instead. The rerender waiter includes `bim_models` with `bim_objects` and `bim_representations`.
