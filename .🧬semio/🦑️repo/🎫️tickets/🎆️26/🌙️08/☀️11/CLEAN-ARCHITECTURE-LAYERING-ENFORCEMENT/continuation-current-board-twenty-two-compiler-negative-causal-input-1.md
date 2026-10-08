# Current Board Twenty Two Compiler Negative Causal Input

Actual released original owning build observed unresolved semio_framework_replication references. Complete raw compiler excerpts follow. Direct source/dispatcher post terminals are pending; no Board native execution or later Root claim is inferred.

```text
   |

error[E0433]: cannot find module or crate `semio_framework_replication` in this scope
  --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧩️composition/🚪️open/🌱️genesis/🦀️.rs:42:10
   |
42 |     crc: semio_framework_replication::codec::Crc32cCursor,
   |          ^^^^^^^^^^^^^^^^^^^^^^^^^^^ use of unresolved module or unlinked crate `semio_framework_replication`
   |
help: there is a crate or module with a similar name
   |
42 -     crc: semio_framework_replication::codec::Crc32cCursor,
42 +     crc: semio_framework_deflate::codec::Crc32cCursor,
   |
help: consider importing one of these modules
   |
 3 + use crate::codec;
   |
 3 + use pack::codec;
   |
 3 + use protocol::codec;
   |
help: if you import `codec`, refer to it directly
   |
42 -     crc: semio_framework_replication::codec::Crc32cCursor,
42 +     crc: codec::Crc32cCursor,
   |

error[E0433]: cannot find module or crate `semio_framework_replication` in this scope
  --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧩️composition/🚪️open/🌱️genesis/🦀️.rs:65:293
   |

   |

error[E0433]: cannot find module or crate `semio_framework_replication` in this scope
  --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧩️composition/🚪️open/🌱️genesis/🦀️.rs:65:293
   |
65 | ...ount: 0, crc: semio_framework_replication::codec::Crc32cCursor::new(), frame_hash: semio_framework_hash::Hasher::new(), chain_has...
   |                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^ use of unresolved module or unlinked crate `semio_framework_replication`
   |
help: there is a crate or module with a similar name
   |
65 -         Self { uri: ManuallyDrop::new(None), source_seal: None, ids: [IdPlan::default(); 8], dictionary: [0; 8], dictionary_len: 0, initial_dictionary_len: 0, field: 0, comparison: 0, comparison_byte: 0, output_position: 0, record: 0, frame_position: 0, records_len: 0, record_count: 0, crc: semio_framework_replication::codec::Crc32cCursor::new(), frame_hash: semio_framework_hash::Hasher::new(), chain_hash: semio_framework_hash::Hasher::new(), closed: false }
65 +         Self { uri: ManuallyDrop::new(None), source_seal: None, ids: [IdPlan::default(); 8], dictionary: [0; 8], dictionary_len: 0, initial_dictionary_len: 0, field: 0, comparison: 0, comparison_byte: 0, output_position: 0, record: 0, frame_position: 0, records_len: 0, record_count: 0, crc: semio_framework_deflate::codec::Crc32cCursor::new(), frame_hash: semio_framework_hash::Hasher::new(), chain_hash: semio_framework_hash::Hasher::new(), closed: false }
   |
help: consider importing one of these structs
   |
 3 + use crate::Crc32cCursor;
   |
 3 + use pack::Crc32cCursor;
   |
 3 + use protocol::Crc32cCursor;
   |
help: if you import `Crc32cCursor`, refer to it directly
   |
65 -         Self { uri: ManuallyDrop::new(None), source_seal: None, ids: [IdPlan::default(); 8], dictionary: [0; 8], dictionary_len: 0, initial_dictionary_len: 0, field: 0, comparison: 0, comparison_byte: 0, output_position: 0, record: 0, frame_position: 0, records_len: 0, record_count: 0, crc: semio_framework_replication::codec::Crc32cCursor::new(), frame_hash: semio_framework_hash::Hasher::new(), chain_hash: semio_framework_hash::Hasher::new(), closed: false }
65 +         Self { uri: ManuallyDrop::new(None), source_seal: None, ids: [IdPlan::default(); 8], dictionary: [0; 8], dictionary_len: 0, initial_dictionary_len: 0, field: 0, comparison: 0, comparison_byte: 0, output_position: 0, record: 0, frame_position: 0, records_len: 0, record_count: 0, crc: Crc32cCursor::new(), frame_hash: semio_framework_hash::Hasher::new(), chain_hash: semio_framework_hash::Hasher::new(), closed: false }
   |

error[E0433]: cannot find module or crate `semio_framework_replication` in this scope
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧩️composition/🚪️open/🌱️genesis/🦀️.rs:159:54
    |

   |

error[E0433]: cannot find module or crate `semio_framework_replication` in this scope
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧩️composition/🚪️open/🌱️genesis/🦀️.rs:159:54
    |
159 | ... { self.crc = semio_framework_replication::codec::Crc32cCursor::new(); self.frame_hash = semio_framework_hash::Hasher::new(); }
    |                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^ use of unresolved module or unlinked crate `semio_framework_replication`
    |
help: there is a crate or module with a similar name
    |
159 -             if self.frame_position == 0 { self.crc = semio_framework_replication::codec::Crc32cCursor::new(); self.frame_hash = semio_framework_hash::Hasher::new(); }
159 +             if self.frame_position == 0 { self.crc = semio_framework_deflate::codec::Crc32cCursor::new(); self.frame_hash = semio_framework_hash::Hasher::new(); }
    |
help: consider importing one of these structs
    |
  3 + use crate::Crc32cCursor;
    |
  3 + use pack::Crc32cCursor;
    |
  3 + use protocol::Crc32cCursor;
    |
help: if you import `Crc32cCursor`, refer to it directly
    |
159 -             if self.frame_position == 0 { self.crc = semio_framework_replication::codec::Crc32cCursor::new(); self.frame_hash = semio_framework_hash::Hasher::new(); }
159 +             if self.frame_position == 0 { self.crc = Crc32cCursor::new(); self.frame_hash = semio_framework_hash::Hasher::new(); }
    |

warning: unnecessary qualification
  --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/././../../🔨️modules/🗣️dsl/👪️family/📊️sheet/🦀️.rs:55:9
   |
```
