"""🏷️ TS twin of the `verb` pass: the wire envelope field and both trailing-flag codecs (bit 0 transaction, bit 1 verb)."""
import pathlib

P = pathlib.Path("/Users/ueli/Documents/semio/🧰️framework/🔨️modules/📡️replication/🟦️.ts")
text = P.read_text()
pairs = [
    ("""  readonly transaction?: TransactionRef;
};

/** 📦️ Owned interface""", """  readonly transaction?: TransactionRef;
  readonly verb?: string;
};

/** 📦️ Owned interface"""),
    ("""    transaction: envelope.transaction ?? null,
  };
}""", """    transaction: envelope.transaction ?? null,
    verb: envelope.verb ?? null,
  };
}"""),
    ("""    ...(envelope.transaction === null ? {} : { transaction: envelope.transaction }),
  };
}""", """    ...(envelope.transaction === null ? {} : { transaction: envelope.transaction }),
    ...(envelope.verb === null ? {} : { verb: envelope.verb }),
  };
}"""),
    ("""  /** 🧾️ The committed tool transaction that authored the operation (`null`: none, and always for a transition). */
  readonly transaction: TransactionRef | null;
};""", """  /** 🧾️ The committed tool transaction that authored the operation (`null`: none, and always for a transition). */
  readonly transaction: TransactionRef | null;
  /** 🏷️ The id of the action or command whose edit carried the operation, never display text (`null`: none, and always
   * for a transition) — a peer resolves it through the authoring app's registry to the same history label. */
  readonly verb: string | null;
};"""),
    ("""/** 🎯️ `mutation_id str | document_id str | actor str | dependencies vec<str> | observed (0 | 1 str) |
 * target vec<str> | diff.schema str | diff.payload bytes | inverse.schema str | inverse.payload bytes | hlc |
 * transaction (0 | 1 id str tool str)` — the TS twin of Rust `protocol_causal::encode_envelope`. */""", """/** 🎯️ `mutation_id str | document_id str | actor str | dependencies vec<str> | observed (0 | 1 str) |
 * target vec<str> | diff.schema str | diff.payload bytes | inverse.schema str | inverse.payload bytes | hlc |
 * trailing flags varint (bit 0 transaction, bit 1 verb) | [transaction id str tool str] | [verb str]` — the TS twin of
 * Rust `protocol_causal::encode_envelope`. */"""),
    ("""  hlc();
  if (envelope.transaction === null) writeVarintU64(out, 0);
  else {
    writeVarintU64(out, 1);
    writeStr(out, envelope.transaction.id);
    writeStr(out, envelope.transaction.tool);
  }
}""", """  hlc();
  writeVarintU64(out, (envelope.transaction === null ? 0 : 1) | (envelope.verb === null ? 0 : 2));
  if (envelope.transaction !== null) {
    writeStr(out, envelope.transaction.id);
    writeStr(out, envelope.transaction.tool);
  }
  if (envelope.verb !== null) writeStr(out, envelope.verb);
}"""),
    ("""  const transactionFlag = readVarintU64(bytes, pos);
  if (transactionFlag !== 0 && transactionFlag !== 1) throw new Error(`mutation envelope: transaction flag ${transactionFlag}`);
  const transaction = transactionFlag === 1 ? { id: readStr(bytes, pos), tool: readStr(bytes, pos) } : null;
  return { mutation_id, document_id, actor, dependencies, observed, target, diff: { schema: diffSchema, payload: diffPayload }, inverse: { schema: inverseSchema, payload: inversePayload }, timestamp, transaction };""", """  const flags = readVarintU64(bytes, pos);
  if (flags > 0b11) throw new Error(`mutation envelope: trailing flags ${flags}`);
  const transaction = (flags & 0b01) !== 0 ? { id: readStr(bytes, pos), tool: readStr(bytes, pos) } : null;
  const verb = (flags & 0b10) !== 0 ? readStr(bytes, pos) : null;
  return { mutation_id, document_id, actor, dependencies, observed, target, diff: { schema: diffSchema, payload: diffPayload }, inverse: { schema: inverseSchema, payload: inversePayload }, timestamp, transaction, verb };"""),
    ("""  timestamp: Readonly<{ actor: bigint; physical_ms: bigint; logical: bigint }>;
  transaction: TransactionRef | null;
}>;""", """  timestamp: Readonly<{ actor: bigint; physical_ms: bigint; logical: bigint }>;
  transaction: TransactionRef | null;
  verb: string | null;
}>;"""),
    ("""    if (envelope.transaction === null) documentBackboneWriteU64(out, 0n);
    else {
      documentBackboneWriteU64(out, 1n);
      documentBackboneWriteText(out, envelope.transaction.id);
      documentBackboneWriteText(out, envelope.transaction.tool);
    }""", """    documentBackboneWriteU64(out, (envelope.transaction === null ? 0n : 1n) | (envelope.verb === null ? 0n : 2n));
    if (envelope.transaction !== null) {
      documentBackboneWriteText(out, envelope.transaction.id);
      documentBackboneWriteText(out, envelope.transaction.tool);
    }
    if (envelope.verb !== null) documentBackboneWriteText(out, envelope.verb);"""),
    ("""    const transactionFlag = readU64();
    if (transactionFlag > 1n) throw new DocumentBackboneBatchError("malformed", "transaction-flag");
    const transaction = transactionFlag === 1n ? { id: readText(limits.maximumIdentifierBytes, "identifier-bytes"), tool: readText(limits.maximumIdentifierBytes, "identifier-bytes") } : null;""", """    const flags = readU64();
    if (flags > 0b11n) throw new DocumentBackboneBatchError("malformed", "trailing-flags");
    const transaction = (flags & 0b01n) !== 0n ? { id: readText(limits.maximumIdentifierBytes, "identifier-bytes"), tool: readText(limits.maximumIdentifierBytes, "identifier-bytes") } : null;
    const verb = (flags & 0b10n) !== 0n ? readText(limits.maximumIdentifierBytes, "identifier-bytes") : null;"""),
    ("""      timestamp,
      transaction,
    });
  }
  if (terminal && position[0] !== bytes.length) throw new DocumentBackboneBatchError("malformed", "trailing-bytes");""", """      timestamp,
      transaction,
      verb,
    });
  }
  if (terminal && position[0] !== bytes.length) throw new DocumentBackboneBatchError("malformed", "trailing-bytes");"""),
]
for old, new in pairs:
    if text.count(new) == 1:
        continue
    count = text.count(old)
    assert count == 1, (count, old[:160])
    text = text.replace(old, new)
P.write_text(text)
print("ok")
