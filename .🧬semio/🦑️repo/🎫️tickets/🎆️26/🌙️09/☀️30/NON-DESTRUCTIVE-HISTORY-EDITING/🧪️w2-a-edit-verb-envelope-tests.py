"""🏷️ Envelope laws and TS literals for the trailing `verb` flag (Rust unit laws, the TS codec laws, every TS envelope
literal)."""
import pathlib

R = pathlib.Path("/Users/ueli/Documents/semio")


def patch(rel, pairs):
    path = R / rel
    text = path.read_text()
    for old, new in pairs:
        if text.count(new) == 1:
            continue
        assert text.count(old) == 1, (rel, text.count(old), old[:120])
        text = text.replace(old, new)
    path.write_text(text)


patch("🧰️framework/🔨️modules/📡️replication/🔗️causal/🧪️tests/🔬️unit/🦀️.rs", [
    ("""/// 🧾️ A tool transaction rides the binary envelope and its value shape; an invalid flag is refused.
#[test]
fn envelope_transaction_round_trips_through_binary_and_value() {
    let mut envelope = sample_envelope("operation-1", vec!["operation-0"]);
    envelope.transaction = Some(crate::mutation::TransactionRef { id: "tx-0123456789abcdef".into(), tool: "app#select".into() });
    let mut out = Vec::new();
    encode_envelope(&envelope, &mut out);
    let mut pos = 0;
    assert_eq!(decode_envelope(&out, &mut pos).expect("decode"), envelope);
    assert_eq!(pos, out.len());
    let decoded: MutationEnvelope = crate::value::FromValue::from_value(crate::value::ToValue::to_value(&envelope)).expect("value decode");
    assert_eq!(decoded, envelope);
    let mut plain = Vec::new();
    encode_envelope(&sample_envelope("operation-1", vec!["operation-0"]), &mut plain);
    *plain.last_mut().expect("transaction flag") = 2;
    assert!(format!("{:?}", decode_envelope(&plain, &mut 0).expect_err("flag 2")).contains("transaction flag 2"));
}""", """/// 🧾️ A tool transaction and an authoring verb ride the binary envelope (trailing flags bit 0 and bit 1) and its value
/// shape, alone and together; an unknown trailing flag is refused.
#[test]
fn envelope_transaction_and_verb_round_trip_through_binary_and_value() {
    let transaction = Some(crate::mutation::TransactionRef { id: "tx-0123456789abcdef".into(), tool: "app#select".into() });
    for (transaction, verb) in [(transaction.clone(), None), (None, Some("typeText".to_string())), (transaction, Some("typeText".to_string()))] {
        let mut envelope = sample_envelope("operation-1", vec!["operation-0"]);
        envelope.transaction = transaction;
        envelope.verb = verb;
        let mut out = Vec::new();
        encode_envelope(&envelope, &mut out);
        let mut pos = 0;
        assert_eq!(decode_envelope(&out, &mut pos).expect("decode"), envelope);
        assert_eq!(pos, out.len());
        let decoded: MutationEnvelope = crate::value::FromValue::from_value(crate::value::ToValue::to_value(&envelope)).expect("value decode");
        assert_eq!(decoded, envelope);
    }
    let mut plain = Vec::new();
    encode_envelope(&sample_envelope("operation-1", vec!["operation-0"]), &mut plain);
    *plain.last_mut().expect("trailing flags") = 4;
    assert!(format!("{:?}", decode_envelope(&plain, &mut 0).expect_err("flag 4")).contains("trailing flags 4"));
}"""),
    ("""                    let transaction = &expected["transaction"];
                    assert_eq!(actual.transaction, (!transaction.is_null()).then(|| crate::mutation::TransactionRef { id: transaction["id"].as_str().expect("transaction id").into(), tool: transaction["tool"].as_str().expect("transaction tool").into() }), "{}", row["id"]);""", """                    let transaction = &expected["transaction"];
                    assert_eq!(actual.transaction, (!transaction.is_null()).then(|| crate::mutation::TransactionRef { id: transaction["id"].as_str().expect("transaction id").into(), tool: transaction["tool"].as_str().expect("transaction tool").into() }), "{}", row["id"]);
                    assert_eq!(actual.verb.as_deref(), expected["verb"].as_str(), "{}", row["id"]);"""),
])

patch("🧰️framework/🔨️modules/📡️replication/🧪️tests/🧪️transaction-ref/🟦️.ts", [
    ("""      for (const value of [envelope, { ...envelope, transaction: null }]) {""", """      for (const value of [envelope, { ...envelope, transaction: null }, { ...envelope, verb: "select" }, { ...envelope, transaction: null, verb: "select" }]) {"""),
    ("""      writeVecEnvelope(out, [{ ...envelope, transaction: null }]);
      out[out.length - 1] = 2;
      expect(() => readVecEnvelope(new Uint8Array(out), [0])).toThrow("transaction flag 2");""", """      writeVecEnvelope(out, [{ ...envelope, transaction: null }]);
      out[out.length - 1] = 4;
      expect(() => readVecEnvelope(new Uint8Array(out), [0])).toThrow("trailing flags 4");"""),
    ("""        transaction: mintTransactionRef("alice", { actor: 1, physical_ms: 2, logical: 3 }, "app#select"),
      };""", """        transaction: mintTransactionRef("alice", { actor: 1, physical_ms: 2, logical: 3 }, "app#select"),
        verb: null as string | null,
      };"""),
])

patch("🧰️framework/🔨️modules/📡️replication/🧪️tests/🧪️document-backbone-envelope-batch/🟦️.ts", [
    ("""    transaction: Readonly<{ id: string; tool: string }> | null;
  }>;""", """    transaction: Readonly<{ id: string; tool: string }> | null;
    verb: string | null;
  }>;"""),
    ("""      transaction: envelope.transaction,
    }));""", """      transaction: envelope.transaction,
      verb: envelope.verb,
    }));"""),
    ("""        timestamp: { actor: 0xfedc_ba98_7654_3210n, physical_ms: (1n << 53n) + 1n, logical: 1n << 60n },
        transaction: null,
      } as const;""", """        timestamp: { actor: 0xfedc_ba98_7654_3210n, physical_ms: (1n << 53n) + 1n, logical: 1n << 60n },
        transaction: null,
        verb: null,
      } as const;"""),
])
patch("🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/👷️worker/🟦️.ts", [
    ("""    timestamp,
    transaction: envelope.transaction ?? null,
  };
}""", """    timestamp,
    transaction: envelope.transaction ?? null,
    verb: envelope.verb ?? null,
  };
}"""),
    ("""    transaction: envelope.transaction,
  };
}""", """    transaction: envelope.transaction,
    verb: envelope.verb,
  };
}"""),
    ("""      baseVersion: sequenceNumber,
      dependencies: [],
      undoPolicy: "exactBaseOnly",
    },
  };
}""", """      baseVersion: sequenceNumber,
      dependencies: [],
      undoPolicy: "exactBaseOnly",
    },
    ...(envelope.transaction === null ? {} : { transaction: envelope.transaction }),
    ...(envelope.verb === null ? {} : { verb: envelope.verb }),
  };
}"""),
    ("""    inverse: { targetOperation: envelope.mutation_id, inverseDiff: { schemaId: envelope.inverse.schema, payload: envelope.inverse.payload.slice() }, baseVersion: 0, dependencies: [], undoPolicy: "exactBaseOnly" },
  };
}""", """    inverse: { targetOperation: envelope.mutation_id, inverseDiff: { schemaId: envelope.inverse.schema, payload: envelope.inverse.payload.slice() }, baseVersion: 0, dependencies: [], undoPolicy: "exactBaseOnly" },
    ...(envelope.transaction === null ? {} : { transaction: envelope.transaction }),
    ...(envelope.verb === null ? {} : { verb: envelope.verb }),
  };
}"""),
])
print("ok")
