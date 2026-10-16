PLAN = dict(
    name="resident",
    orig="svm-tools/orig/resident.rs",
    real="🧰️framework/🔨️modules/🌱️value/💾️resident/🦀️.rs",
    real_lib="🧰️framework/🔨️modules/🌱️value/💾️resident/🦀️.rs",
    mount_after=221,
    children=[
        dict(key="consumer", folder="🪪️consumer-cell", mod="consumer_cell", doc="🪪️ Consumer cells and admissions: consumer pages, typed consumer handles, ledger admission and consumer-side release.", ranges=[(828, 1608)]),
    ],
    oracle=dict(trees=["🧰️framework/🔨️modules/🌱️value/💾️resident"], crate="semio_framework_value_resident", cfgs=["lib", "test", "wasm", "p2"]),
    no_widen=["field ConsumerHeader.type_id", "field ConsumerHeader.registration", "field ConsumerNode.source", "field ResidentConsumerRead.pointer", "field ErasedConsumer.pointer", "field ErasedConsumer.empty", "field ResidentConsumer.pointer"],
)
