extern crate semio_framework_graph as graph_core;
extern crate self as geometry;

pub mod random {
    #[derive(Clone, Copy)]
    pub struct Rng { state: [u64; 4] }

    impl Rng {
        pub fn from_seed(seed: u64) -> Self {
            let mut cursor = seed;
            let mut state = [0; 4];
            for word in &mut state {
                cursor = cursor.wrapping_add(0x9e37_79b9_7f4a_7c15);
                let mut z = cursor;
                z = (z ^ (z >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
                z = (z ^ (z >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
                *word = z ^ (z >> 31);
            }
            Self { state }
        }
        pub fn from_state(state: [u64; 4]) -> Self { Self { state } }
        pub fn state(&self) -> [u64; 4] { self.state }
        pub fn next_u64(&mut self) -> u64 {
            let result = self.state[1].wrapping_mul(5).rotate_left(7).wrapping_mul(9);
            let t = self.state[1] << 17;
            self.state[2] ^= self.state[0]; self.state[3] ^= self.state[1];
            self.state[1] ^= self.state[2]; self.state[0] ^= self.state[3];
            self.state[2] ^= t; self.state[3] = self.state[3].rotate_left(45);
            result
        }
        pub fn next_f64(&mut self) -> f64 { (self.next_u64() >> 11) as f64 * (1.0 / (1u64 << 53) as f64) }
        pub fn next_range(&mut self, lo: u64, hi: u64) -> u64 { lo + self.next_u64() % (hi - lo).max(1) }
        pub fn next_bool(&mut self, probability: f64) -> bool { self.next_f64() < probability }
        pub fn shuffle<T>(&mut self, values: &mut [T]) {
            for i in (1..values.len()).rev() { let j = self.next_range(0, (i + 1) as u64) as usize; values.swap(i, j); }
        }
        pub fn choose<'a, T>(&mut self, values: &'a [T]) -> Option<&'a T> {
            (!values.is_empty()).then(|| &values[self.next_range(0, values.len() as u64) as usize])
        }
        pub fn sample_without_replacement(&mut self, n: usize, k: usize) -> Vec<usize> {
            let mut values: Vec<_> = (0..n).collect(); self.shuffle(&mut values); values.truncate(k); values
        }
    }
}

pub mod wfc_engine {
    #[path = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧩️assembly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧩️wfc-engine/🔦️beam/🦀️component.rs"]
    pub mod beam;
    #[path = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧩️assembly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧩️wfc-engine/🎛️bitset/🦀️component.rs"]
    pub mod bitset;
    #[path = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧩️assembly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧩️wfc-engine/🍰️chunk/🦀️component.rs"]
    pub mod chunk;
    #[path = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧩️assembly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧩️wfc-engine/⛓️constraint/🦀️component.rs"]
    pub mod constraint;
    #[path = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧩️assembly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧩️wfc-engine/🔢️constraints-card/🦀️component.rs"]
    pub mod constraints_card;
    #[path = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧩️assembly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧩️wfc-engine/🔗️constraints-conn/🦀️component.rs"]
    pub mod constraints_conn;
    #[path = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧩️assembly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧩️wfc-engine/🩺️diag/🦀️component.rs"]
    pub mod diag;
    #[path = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧩️assembly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧩️wfc-engine/🌐️domain/🦀️component.rs"]
    pub mod domain;
    #[path = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧩️assembly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧩️wfc-engine/⚠️error/🦀️component.rs"]
    pub mod error;
    #[path = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧩️assembly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧩️wfc-engine/🧬️evolve/🦀️component.rs"]
    pub mod evolve;
    #[path = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧩️assembly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧩️wfc-engine/⛏️extract/🦀️component.rs"]
    pub mod extract;
    #[path = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧩️assembly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧩️wfc-engine/🌊️flow/🦀️component.rs"]
    pub mod flow;
    #[path = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧩️assembly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧩️wfc-engine/🔲️grid-2d/🦀️component.rs"]
    pub mod grid2d;
    #[path = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧩️assembly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧩️wfc-engine/🧊️grid-3d/🦀️component.rs"]
    pub mod grid3d;
    #[path = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧩️assembly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧩️wfc-engine/🧭️heuristics/🦀️component.rs"]
    pub mod heuristics;
    #[path = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧩️assembly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧩️wfc-engine/🪜️hierarchy/🦀️component.rs"]
    pub mod hierarchy;
    #[path = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧩️assembly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧩️wfc-engine/🆔️ids/🦀️component.rs"]
    pub mod ids;
    #[path = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧩️assembly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧩️wfc-engine/🧵️job/🦀️component.rs"]
    pub mod job;
    #[path = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧩️assembly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧩️wfc-engine/🏗️model/🦀️component.rs"]
    pub mod model;
    #[path = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧩️assembly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧩️wfc-engine/🎼️motif/🦀️component.rs"]
    pub mod motif;
    #[path = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧩️assembly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧩️wfc-engine/🚫️nogood/🦀️component.rs"]
    pub mod nogood;
    #[path = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧩️assembly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧩️wfc-engine/🔮️oracle/🦀️component.rs"]
    pub mod oracle;
    #[path = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧩️assembly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧩️wfc-engine/🏁️outcome/🦀️component.rs"]
    pub mod outcome;
    #[path = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧩️assembly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧩️wfc-engine/🧵️parallel/🦀️component.rs"]
    pub mod parallel;
    #[path = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧩️assembly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧩️wfc-engine/🔁️prop-ac3/🦀️component.rs"]
    pub mod prop_ac3;
    #[path = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧩️assembly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧩️wfc-engine/🔄️prop-ac4/🦀️component.rs"]
    pub mod prop_ac4;
    #[path = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧩️assembly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧩️wfc-engine/📣️propagate/🦀️component.rs"]
    pub mod propagate;
    #[path = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧩️assembly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧩️wfc-engine/🔧️repair/🦀️component.rs"]
    pub mod repair;
    #[path = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧩️assembly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧩️wfc-engine/🎲️sample/🦀️component.rs"]
    pub mod sample;
    #[path = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧩️assembly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧩️wfc-engine/🔍️search/🦀️component.rs"]
    pub mod search;
    #[path = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧩️assembly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧩️wfc-engine/💾️serial/🦀️component.rs"]
    pub mod serial;
    #[path = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧩️assembly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧩️wfc-engine/🪶️soft/🦀️component.rs"]
    pub mod soft;
    #[path = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧩️assembly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧩️wfc-engine/🕸️solver-graph/🦀️component.rs"]
    pub mod solver_graph;
    #[path = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧩️assembly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧩️wfc-engine/🔳️solver-grid-2d/🦀️component.rs"]
    pub mod solver_grid2d;
    #[path = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧩️assembly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧩️wfc-engine/🧱️solver-grid-3d/🦀️component.rs"]
    pub mod solver_grid3d;
    #[path = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧩️assembly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧩️wfc-engine/🕳️sparse-3d/🦀️component.rs"]
    pub mod sparse3d;
    #[path = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧩️assembly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧩️wfc-engine/🪞️symmetry/🦀️component.rs"]
    pub mod symmetry;
    #[path = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧩️assembly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧩️wfc-engine/🀄️tiled/🦀️component.rs"]
    pub mod tiled;
    #[path = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧩️assembly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧩️wfc-engine/🗺️topology/🦀️component.rs"]
    pub mod topology;
    #[path = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧩️assembly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧩️wfc-engine/🐾️trail/🦀️component.rs"]
    pub mod trail;
    #[path = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧩️assembly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧩️wfc-engine/⚖️weights/🦀️component.rs"]
    pub mod weights;
}

