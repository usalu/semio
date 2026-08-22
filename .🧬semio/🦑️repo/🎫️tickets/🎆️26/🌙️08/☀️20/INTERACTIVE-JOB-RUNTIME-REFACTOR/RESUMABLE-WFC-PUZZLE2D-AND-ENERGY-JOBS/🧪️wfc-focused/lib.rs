extern crate semio_framework_graph as graph_core;
extern crate self as geometry;

pub mod random {
    pub struct Rng(u64);

    impl Rng {
        pub fn from_seed(seed: u64) -> Self {
            Self(seed)
        }

        pub fn next_u64(&mut self) -> u64 {
            self.0 ^= self.0 << 13;
            self.0 ^= self.0 >> 7;
            self.0 ^= self.0 << 17;
            self.0
        }

        pub fn next_range(&mut self, lo: u64, hi: u64) -> u64 {
            lo + self.next_u64() % (hi - lo).max(1)
        }

        pub fn next_bool(&mut self, probability: f64) -> bool {
            (self.next_u64() >> 11) as f64 * (1.0 / (1u64 << 53) as f64) < probability
        }
    }
}

pub mod wfc_engine {
    #[path = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧩️assembly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧩️wfc-engine/🎛️bitset/🦀️component.rs"]
    pub mod bitset;
    #[path = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧩️assembly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧩️wfc-engine/⚠️error/🦀️component.rs"]
    pub mod error;
    #[path = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧩️assembly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧩️wfc-engine/🆔️ids/🦀️component.rs"]
    pub mod ids;
    #[path = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧩️assembly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧩️wfc-engine/🧵️job/🦀️component.rs"]
    pub mod job;
    #[path = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧩️assembly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧩️wfc-engine/🏗️model/🦀️component.rs"]
    pub mod model;
    #[path = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧩️assembly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧩️wfc-engine/🗺️topology/🦀️component.rs"]
    pub mod topology;
    #[path = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧩️assembly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧩️wfc-engine/⚖️weights/🦀️component.rs"]
    pub mod weights;
}
