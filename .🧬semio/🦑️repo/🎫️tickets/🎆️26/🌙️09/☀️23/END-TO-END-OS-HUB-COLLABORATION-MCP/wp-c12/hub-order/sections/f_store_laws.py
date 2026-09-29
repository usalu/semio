# 🧪️ Section F: the store's hub-order laws — a language-agnostic fixture (deterministic scenarios plus the seeded property's
# parameters) and its schema, replayed by N demo stores on `ChannelBackbone`s through a test sequencer that hands them the hub's
# order exactly as the worker and the sync actors do.
HUB_ORDER_FIXTURE = STORE_DIR + "🧫️fixtures/🧭️hub-order/🔣️.json"
HUB_ORDER_SCHEMA = STORE_DIR + "🧬️schema/🧭️hub-order/🔣️.json"


def op_step(kind, replica, label, **operation):
    return {kind: {"replica": replica, "label": label, **operation}}


HUB_ORDER = {
    "$schema": "../../🧬️schema/🧭️hub-order/🔣️.json",
    "schema": "semio.store.hub-order/v1",
    "description": "Replicas of one document converge on the order its sequencer (the hub) committed their operations in. A replica folds the sequencer's operations (`Sequenced`) right before its own operations the sequencer has not decided yet — splitting a typing run the sequencer interrupted — and replays those on top; the sequencer's `Committed` decides them where they stand. Every entry is stamped along the sequencer's order (its own stamp when later, else the next tick after the entry the sequencer placed before it) and undecided entries move past every placed one, so the history fold — which orders by stamp — keeps the sequencer's order through every local edit. Every replica ends byte-identical to the hub's log: its applied operation ids equal the log, its state equals the log's fold, every operation carries the same stamp on every replica.",
    "artifactSchema": "demo/v1",
    "documentId": "demo",
    "genesisN": 0,
    "scenarios": [
        {
            "id": "both-type-at-the-same-point",
            "replicas": ["a", "b"],
            "steps": [op_step("type", "a", "a1", add=1), op_step("type", "b", "b1", set=5), {"sequence": "b"}, {"sequence": "a"}, {"deliver": "a"}, {"deliver": "b"}],
            "expect": {"order": ["b1", "a1"], "n": 6, "appliedEdits": {"a": 2, "b": 2}},
        },
        {
            "id": "a-typing-run-split-by-a-peer",
            "replicas": ["a", "b"],
            "steps": [op_step("type", "a", "a1", add=1), {"sequence": "a"}, op_step("type", "b", "b1", set=50), {"sequence": "b"}, op_step("type", "a", "a2", add=100), {"deliver": "a"}, {"sequence": "a"}, {"deliver": "a"}, {"deliver": "b"}],
            "expect": {"order": ["a1", "b1", "a2"], "n": 150, "appliedEdits": {"a": 3, "b": 3}},
        },
        {
            "id": "three-replicas-in-the-hubs-order",
            "replicas": ["a", "b", "c"],
            "steps": [op_step("apply", "a", "a1", set=1), op_step("type", "b", "b1", add=2), op_step("type", "c", "c1", add=3), {"sequence": "c"}, {"sequence": "a"}, {"sequence": "b"}, {"deliver": "b"}, {"deliver": "c"}, {"deliver": "a"}],
            "expect": {"order": ["c1", "a1", "b1"], "n": 3, "appliedEdits": {"a": 3, "b": 3, "c": 3}},
        },
        {
            "id": "an-offline-run-rebases-after-reconnect",
            "replicas": ["a", "b"],
            "steps": [{"cut": "a"}, op_step("type", "a", "a1", add=1), op_step("type", "a", "a2", add=1), op_step("type", "b", "b1", set=7), {"sequence": "b"}, {"sequence": "a"}, {"deliver": "a"}, {"restore": "a"}, {"deliver": "a"}, {"sequence": "a"}, {"deliver": "a"}, {"deliver": "b"}],
            "expect": {"order": ["b1", "a1", "a2"], "n": 9, "appliedEdits": {"a": 2, "b": 3}},
        },
        {
            "id": "a-local-edit-after-a-rebase-keeps-the-hubs-order",
            "replicas": ["a", "b"],
            "steps": [op_step("type", "a", "a1", add=1), op_step("apply", "b", "b1", set=10), {"sequence": "b"}, {"deliver": "a"}, op_step("apply", "a", "a2", add=100), {"sequence": "a"}, {"deliver": "a"}, {"deliver": "b"}],
            "expect": {"order": ["b1", "a1", "a2"], "n": 111, "appliedEdits": {"a": 3, "b": 3}},
        },
        {
            "id": "a-peer-edit-lands-after-decided-ones",
            "replicas": ["a", "b"],
            "steps": [op_step("type", "a", "a1", set=3), {"sequence": "a"}, {"deliver": "a"}, op_step("type", "b", "b1", add=4), {"sequence": "b"}, {"deliver": "a"}, {"deliver": "b"}],
            "expect": {"order": ["a1", "b1"], "n": 7, "appliedEdits": {"a": 2, "b": 2}},
        },
    ],
    "property": {
        "description": "Seeded interleavings of concurrent typing at the same point, separate edits, link cuts and reconnects across the replicas; after quiescence every replica equals the hub's log.",
        "seeds": 128,
        "replicas": 3,
        "actions": 48,
        "maximumOperationsPerReplica": 8,
        "percent": {"type": 35, "apply": 10, "sequence": 20, "deliver": 25, "cut": 10},
    },
}

OPERATION = {"type": "object", "additionalProperties": False, "required": ["replica", "label"], "properties": {"replica": {"type": "string", "minLength": 1}, "label": {"type": "string", "minLength": 1}, "set": {"type": "integer"}, "add": {"type": "integer"}}, "oneOf": [{"required": ["set"]}, {"required": ["add"]}]}
HUB_ORDER_SCHEMA_JSON = {
    "$schema": "http://json-schema.org/draft-07/schema#",
    "$id": "https://json.schemas.assets.semio-tech.com/os/store/hub-order/v1.json",
    "title": "Store Hub Order V1",
    "description": "Replicas of one document converge on its sequencer's order: scenarios of typed and applied operations, sequencer turns, deliveries and link cuts, each with the hub order every replica must end with, plus the seeded property's parameters.",
    "type": "object",
    "additionalProperties": False,
    "required": ["schema", "description", "artifactSchema", "documentId", "genesisN", "scenarios", "property"],
    "properties": {
        "$schema": {"type": "string"},
        "schema": {"const": "semio.store.hub-order/v1"},
        "description": {"type": "string"},
        "artifactSchema": {"type": "string", "minLength": 1},
        "documentId": {"type": "string", "minLength": 1},
        "genesisN": {"type": "integer"},
        "scenarios": {"type": "array", "minItems": 1, "items": {"$ref": "#/$defs/Scenario"}},
        "property": {"$ref": "#/$defs/Property"},
    },
    "$defs": {
        "Operation": OPERATION,
        "Step": {
            "type": "object",
            "additionalProperties": False,
            "minProperties": 1,
            "maxProperties": 1,
            "properties": {
                "type": {"$ref": "#/$defs/Operation", "description": "Types into the replica's tail typing run (coalesced)."},
                "apply": {"$ref": "#/$defs/Operation", "description": "Applies one operation as its own edit."},
                "sequence": {"type": "string", "description": "The hub takes and commits every operation the replica sent (not while its link is cut)."},
                "deliver": {"type": "string", "description": "The replica folds every message the hub holds for it (not while its link is cut)."},
                "cut": {"type": "string"},
                "restore": {"type": "string"},
            },
        },
        "Scenario": {
            "type": "object",
            "additionalProperties": False,
            "required": ["id", "replicas", "steps", "expect"],
            "properties": {
                "id": {"type": "string", "pattern": "^[a-z0-9]+(-[a-z0-9]+)*$"},
                "replicas": {"type": "array", "minItems": 2, "uniqueItems": True, "items": {"type": "string", "minLength": 1}},
                "steps": {"type": "array", "minItems": 1, "items": {"$ref": "#/$defs/Step"}},
                "expect": {
                    "type": "object",
                    "additionalProperties": False,
                    "required": ["order", "n", "appliedEdits"],
                    "properties": {
                        "order": {"type": "array", "items": {"type": "string"}},
                        "n": {"type": "integer"},
                        "appliedEdits": {"type": "object", "additionalProperties": {"type": "integer", "minimum": 0}},
                    },
                },
            },
        },
        "Property": {
            "type": "object",
            "additionalProperties": False,
            "required": ["description", "seeds", "replicas", "actions", "maximumOperationsPerReplica", "percent"],
            "properties": {
                "description": {"type": "string"},
                "seeds": {"type": "integer", "minimum": 1},
                "replicas": {"type": "integer", "minimum": 2},
                "actions": {"type": "integer", "minimum": 1},
                "maximumOperationsPerReplica": {"type": "integer", "minimum": 1},
                "percent": {
                    "type": "object",
                    "additionalProperties": False,
                    "required": ["type", "apply", "sequence", "deliver", "cut"],
                    "properties": {name: {"type": "integer", "minimum": 0, "maximum": 100} for name in ["type", "apply", "sequence", "deliver", "cut"]},
                },
            },
        },
    },
}

create(HUB_ORDER_FIXTURE, json.dumps(HUB_ORDER, indent=2, ensure_ascii=False) + "\n", "store hub-order fixture")
create(HUB_ORDER_SCHEMA, json.dumps(HUB_ORDER_SCHEMA_JSON, indent=2, ensure_ascii=False) + "\n", "store hub-order schema")

STORE_LAWS_ANCHOR = """    assert_eq!(edit_ids.len() as u64, fixture["expect"]["distinctEditIds"].as_u64().expect("distinct edits"));
    assert_eq!(operation_ids.len() as u64, fixture["expect"]["distinctOperationIds"].as_u64().expect("distinct operations"));
}
"""
HUB_ORDER_LAWS = STORE_LAWS_ANCHOR + """
//#region 🔖️HubOrder
/// 🧭️ One replica of the hub-order laws: a demo store on a `ChannelBackbone`, its transport owner's end, and the messages the
/// hub holds for it (kept while its link is cut, delivered in order after).
struct HubOrderReplica {
    store: ArtifactStore<DemoSnapshot, DemoMutation>,
    remote: ChannelBackboneRemote,
    inbound: VecDeque<BackboneMessage>,
    cut: bool,
}

/// 🏛️ The laws' sequencer: commits each replica's sent operations in the order it takes them and hands every replica the
/// result the way the worker and the sync actors hand a hub's frames to a store — `Committed` to the author, `Sequenced` to
/// every other replica. `log` is the hub's order with each operation.
struct HubOrderHub {
    replicas: Vec<HubOrderReplica>,
    log: Vec<(String, DemoMutation)>,
    rebases: usize,
    splits: usize,
}

impl HubOrderHub {
    async fn open(count: usize) -> Self {
        let mut replicas = Vec::with_capacity(count);
        for index in 0..count {
            let (channel, remote) = ChannelBackbone::pair(&format!("hub-order-{index}")).await;
            let mut store = fresh_demo_store().await;
            store.attach_backbone(Backbones::Channel(channel)).await.expect("a replica attaches its transport owner's channel");
            replicas.push(HubOrderReplica { store, remote, inbound: VecDeque::new(), cut: false });
        }
        Self { replicas, log: Vec::new(), rebases: 0, splits: 0 }
    }

    /// ⌨️ Replica `index` authors `mutation` — `typing` coalesces it into the tail typing run — and answers its operation id.
    async fn author(&mut self, index: usize, mutation: DemoMutation, typing: bool) -> String {
        let store = &mut self.replicas[index].store;
        let command = if typing { ArtifactCommand::AmendLast { mutations: vec![mutation], coalesce_key: Some("typing".into()) } } else { ArtifactCommand::Apply { mutations: vec![mutation], description: None } };
        store.dispatch(command).await.expect("a replica's local edit applies");
        let tail = store.applied_edit_ids().last().expect("the local edit is the applied tail").clone();
        let edit = store.envelope.vcs.edits.iter().find(|edit| edit.id == tail).expect("the tail edit is held");
        crate::os_spr::mutation_ids_for_edit::<DemoSnapshot, DemoMutation>(edit).last().expect("the tail edit holds the new operation").0.clone()
    }

    /// 🏛️ Takes every operation replica `index` sent and commits it, unless its link is cut.
    async fn sequence(&mut self, index: usize) {
        if self.replicas[index].cut {
            return;
        }
        for message in drain_channel_for_test(&self.replicas[index].remote).expect("a replica's outbound drains") {
            let BackboneMessage::Mutations { envelopes } = message else { continue };
            for envelope in crate::os_spr::decode_envelopes(&envelopes).expect("a replica's batch decodes") {
                let operation = edit_from_operation_envelope::<DemoMutation>(&envelope).await.expect("the hub reads the operation").forwards.remove(0);
                let id = envelope.mutation_id.0.clone();
                self.log.push((id.clone(), operation));
                for (other, replica) in self.replicas.iter_mut().enumerate() {
                    replica.inbound.push_back(if other == index { BackboneMessage::Committed { op_ids: vec![id.clone()] } } else { BackboneMessage::Sequenced { envelopes: crate::os_spr::encode_envelopes(std::slice::from_ref(&envelope)) } });
                }
            }
        }
    }

    /// 📬️ Replica `index` folds the first `count` messages the hub holds for it, unless its link is cut. Every `Sequenced`
    /// message of the laws carries one operation, one applied edit; an applied edit beyond those is a split typing run.
    async fn deliver(&mut self, index: usize, count: usize) {
        let HubOrderReplica { store, remote, inbound, cut } = &mut self.replicas[index];
        if *cut {
            return;
        }
        let undecided = !store.backbone.as_ref().expect("attached").unconfirmed_operations().is_empty();
        let (edits, mut sequenced) = (store.applied_edit_ids().len(), 0);
        for message in inbound.drain(..count.min(inbound.len())) {
            let operation = matches!(message, BackboneMessage::Sequenced { .. });
            self.rebases += usize::from(undecided && operation);
            sequenced += usize::from(operation);
            remote.push(message).await.expect("the transport owner hands the store its message");
        }
        store.tick().await.expect("a replica folds the hub's messages");
        self.splits += store.applied_edit_ids().len() - edits - sequenced;
    }

    /// ⏹️ Restores every link, then sequences and delivers until nothing moves.
    async fn quiesce(&mut self) {
        for replica in &mut self.replicas {
            replica.cut = false;
        }
        loop {
            for index in 0..self.replicas.len() {
                self.sequence(index).await;
            }
            if self.replicas.iter().all(|replica| replica.inbound.is_empty()) {
                return;
            }
            for index in 0..self.replicas.len() {
                let count = self.replicas[index].inbound.len();
                self.deliver(index, count).await;
            }
        }
    }

    /// 🧾️ Replica `index`'s applied operation ids, in applied order.
    fn applied_operations(&self, index: usize) -> Vec<String> {
        let store = &self.replicas[index].store;
        store.applied_edit_ids().iter().flat_map(|edit_id| crate::os_spr::mutation_ids_for_edit::<DemoSnapshot, DemoMutation>(store.envelope.vcs.edits.iter().find(|edit| edit.id == *edit_id).expect("an applied edit is held"))).map(|id| id.0).collect()
    }

    /// ⏭️ Replica `index`'s stamp of every operation it holds.
    fn stamps(&self, index: usize) -> HashMap<String, HybridLogicalTimestamp> {
        self.replicas[index].store.envelope.vcs.edits.iter().flat_map(|edit| edit.mutation_meta.iter().filter_map(|meta| meta.mutation_id.as_ref().map(|id| (id.0.clone(), meta.timestamp)))).collect()
    }

    /// 🧮️ Every replica's history fold — which orders entries by stamp, and which every local edit reprojects through —
    /// reproduces its applied order.
    fn assert_fold_agrees(&self, context: &str) {
        for (index, replica) in self.replicas.iter().enumerate() {
            let fold = fold_envelope_history::<DemoSnapshot, DemoMutation>(&replica.store.envelope).expect("the history folds");
            assert_eq!(fold.applied, replica.store.applied_edit_ids().to_vec(), "{context}: replica {index}'s history fold keeps its applied order");
        }
    }

    /// ⚖️ Every replica equals the hub's log: its applied operations, its state (the log's fold), its stamps, nothing left
    /// undecided.
    fn assert_hub_order(&self, context: &str) {
        let log: Vec<String> = self.log.iter().map(|(id, _)| id.clone()).collect();
        let n = self.log.iter().fold(Some(0), |n, (_, operation)| match operation {
            DemoMutation::SetN(SetN { n }) => Some(*n),
            DemoMutation::AddN(AddN { delta }) => n.map(|value: i32| value.saturating_add(*delta)),
            other => panic!("{context}: the laws author only set and add, not {other:?}"),
        });
        for (index, replica) in self.replicas.iter().enumerate() {
            assert_eq!(self.applied_operations(index), log, "{context}: replica {index} folds the hub's order");
            assert_eq!(replica.store.snapshot().expect("replica snapshot").n, n, "{context}: replica {index} holds the fold of the hub's order");
            assert!(replica.store.backbone.as_ref().expect("attached").unconfirmed_operations().is_empty(), "{context}: replica {index} keeps no undecided operation after quiescence");
            assert_eq!(self.stamps(index), self.stamps(0), "{context}: replica {index} stamps every operation the way replica 0 does");
        }
        self.assert_fold_agrees(context);
    }
}

fn hub_order_fixture() -> serde_json::Value {
    serde_json::from_str(include_str!("../../🧫️fixtures/🧭️hub-order/🔣️.json")).expect("hub-order fixture")
}

fn hub_order_operation(operation: &serde_json::Value) -> DemoMutation {
    match (operation.get("set"), operation.get("add")) {
        (Some(n), None) => DemoMutation::SetN(SetN { n: n.as_i64().expect("set n") as i32 }),
        (None, Some(delta)) => DemoMutation::AddN(AddN { delta: delta.as_i64().expect("add delta") as i32 }),
        _ => panic!("a hub-order operation is exactly one of set/add"),
    }
}

/// 🧭️ Fixture law `🧫️fixtures/🧭️hub-order` scenarios: replicas end byte-identical to the hub's order — a peer's operation
/// the hub committed first lands before a replica's own undecided ones (splitting a typing run it interrupted), an offline run
/// rebases after reconnect, and three replicas agree on the sequencer's order, never on their clocks'.
#[semio_framework_async_macros::async_test]
async fn sequenced_replicas_fold_the_hubs_order_in_every_fixture_scenario() {
    let fixture = hub_order_fixture();
    for scenario in fixture["scenarios"].as_array().expect("scenarios") {
        let id = scenario["id"].as_str().expect("scenario id");
        let names: Vec<&str> = scenario["replicas"].as_array().expect("replicas").iter().map(|name| name.as_str().expect("replica name")).collect();
        let replica = |name: &serde_json::Value| names.iter().position(|known| Some(*known) == name.as_str()).unwrap_or_else(|| panic!("{id}: unknown replica {name}"));
        let mut hub = HubOrderHub::open(names.len()).await;
        let mut labels: HashMap<String, String> = HashMap::new();
        for step in scenario["steps"].as_array().expect("steps") {
            let (kind, value) = step.as_object().and_then(|step| step.iter().next()).expect("a step names one action");
            match kind.as_str() {
                "type" | "apply" => {
                    let operation_id = hub.author(replica(&value["replica"]), hub_order_operation(value), kind == "type").await;
                    labels.insert(operation_id, value["label"].as_str().expect("label").to_string());
                }
                "sequence" => hub.sequence(replica(value)).await,
                "deliver" => {
                    let index = replica(value);
                    let count = hub.replicas[index].inbound.len();
                    hub.deliver(index, count).await;
                }
                "cut" => hub.replicas[replica(value)].cut = true,
                "restore" => hub.replicas[replica(value)].cut = false,
                other => panic!("{id}: unknown step {other}"),
            }
            hub.assert_fold_agrees(id);
        }
        let expect = &scenario["expect"];
        let order: Vec<&str> = expect["order"].as_array().expect("order").iter().map(|label| label.as_str().expect("label")).collect();
        assert_eq!(hub.log.iter().map(|(operation, _)| labels[operation].as_str()).collect::<Vec<_>>(), order, "{id}: the hub's order");
        for (index, name) in names.iter().enumerate() {
            let applied: Vec<String> = hub.applied_operations(index).iter().map(|operation| labels[operation].clone()).collect();
            assert_eq!(applied, order, "{id}: replica {name} folds the hub's order before quiescence");
            assert_eq!(hub.replicas[index].store.applied_edit_ids().len() as u64, expect["appliedEdits"][*name].as_u64().expect("applied edits"), "{id}: replica {name}'s undo steps");
        }
        assert_eq!(hub.replicas[0].store.snapshot().expect("snapshot").n, Some(expect["n"].as_i64().expect("n") as i32), "{id}: the fold of the hub's order");
        hub.assert_hub_order(id);
    }
}

/// 🎲️ Seeded property (`🧫️fixtures/🧭️hub-order` `property`): random interleavings of concurrent typing at the same point,
/// separate edits, link cuts and reconnects across N replicas — after every action each replica's history fold keeps its
/// applied order, and after quiescence every replica is byte-identical to the hub's order, stamps included. The run must exercise the rebase: sequenced operations reaching replicas with undecided ones, and split typing runs.
#[semio_framework_async_macros::async_test]
async fn seeded_concurrent_typing_and_reconnects_converge_on_the_hubs_order() {
    let fixture = hub_order_fixture();
    let property = &fixture["property"];
    let count = property["replicas"].as_u64().expect("replicas") as usize;
    let percent = |name: &str| property["percent"][name].as_u64().expect("percent");
    let bounds: Vec<(u64, &str)> = ["type", "apply", "sequence", "deliver", "cut"].iter().scan(0, |total, name| {
        *total += percent(name);
        Some((*total, *name))
    }).collect();
    assert_eq!(bounds.last().map(|(total, _)| *total), Some(100), "the property's percentages cover every roll");
    let maximum = property["maximumOperationsPerReplica"].as_u64().expect("maximum operations") as usize;
    let (mut rebases, mut splits) = (0usize, 0usize);
    for seed in 1..=property["seeds"].as_u64().expect("seeds") {
        let mut state = seed;
        let mut next = move || {
            state = state.wrapping_add(0x9E37_79B9_7F4A_7C15);
            let mut z = state;
            z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
            z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
            z ^ (z >> 31)
        };
        let mut hub = HubOrderHub::open(count).await;
        let mut authored = vec![0usize; count];
        for _ in 0..property["actions"].as_u64().expect("actions") {
            let roll = next() % 100;
            let index = (next() % count as u64) as usize;
            let action = bounds.iter().find(|(total, _)| roll < *total).map(|(_, name)| *name).expect("a roll lands on an action");
            match action {
                "type" | "apply" if authored[index] < maximum => {
                    let mutation = if next() % 4 == 0 { DemoMutation::SetN(SetN { n: (next() % 1_000) as i32 }) } else { DemoMutation::AddN(AddN { delta: 1 + (next() % 9) as i32 }) };
                    hub.author(index, mutation, action == "type").await;
                    authored[index] += 1;
                }
                "sequence" => hub.sequence(index).await,
                "deliver" => {
                    let held = hub.replicas[index].inbound.len();
                    let count = if held == 0 { 0 } else { 1 + (next() as usize % held) };
                    hub.deliver(index, count).await;
                }
                "cut" => hub.replicas[index].cut = !hub.replicas[index].cut,
                _ => {}
            }
            hub.assert_fold_agrees(&format!("seed {seed}"));
        }
        hub.quiesce().await;
        hub.assert_hub_order(&format!("seed {seed}"));
        rebases += hub.rebases;
        splits += hub.splits;
    }
    assert!(rebases > 0 && splits > 0, "the seeded runs exercise the rebase ({rebases} sequenced batches reached undecided operations) and the split ({splits} split typing runs)");
}
//#endregion 🔖️HubOrder
"""
edit(STORE_LAWS, STORE_LAWS_ANCHOR, HUB_ORDER_LAWS, "store hub-order laws")
