#![allow(dead_code, unused_imports)]
extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

#[path = "."]
pub mod standards {
    #[path = "."]
    pub mod v1_7 {
        #[path = "."]
        pub mod subsets {
            #[path = "."]
            pub mod base {
                #[path = "."]
                pub mod schema {
                    #[path = "."]
                    pub mod snapshot {
                        #[path = "../🦀️snapshot-next.rs"]
                        mod component;
                        pub use component::*;
                    }
                    pub use super::diff_holder as diff;
                }
                #[path = "."]
                pub mod diff_holder {
                    #[path = "../🦀️diff-next.rs"]
                    mod component;
                    pub use component::*;
                }
                #[path = "."]
                pub mod io {
                    #[path = "../🦀️io-next.rs"]
                    mod component;
                    pub use component::*;
                }
                #[path = "."]
                pub mod modules {
                    #[path = "../../../../../../../../✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🔨️modules/🔤️lexer/🦀️.rs"]
                    pub mod lexer;
                    #[path = "../../../../../../../../✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🔨️modules/🗜️filters/🦀️.rs"]
                    pub mod filters;
                    #[path = "../../../../../../../../✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🔨️modules/🔗️xref/🦀️.rs"]
                    pub mod xref;
                    #[path = "../../../../../../../../✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🔨️modules/🔐️encryption/🦀️.rs"]
                    pub mod encryption;
                    #[path = "../../../../../../../../✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🔨️modules/🔤️fonts/🦀️.rs"]
                    pub mod fonts;
                    #[path = "../../../../../../../../✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🔨️modules/🖋️content/🦀️.rs"]
                    pub mod content;
                    #[path = "../../../../../../../../✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🔨️modules/🎨️colour/🦀️.rs"]
                    pub mod colour;
                    #[path = "../../../../../../../../✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🔨️modules/🖼️images/🦀️.rs"]
                    pub mod images;
                    #[path = "../../../../../../../../✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🔨️modules/⬇️lift/🦀️.rs"]
                    pub mod lift;
                    #[path = "../../../../../../../../✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🔨️modules/⬆️lower/🦀️.rs"]
                    pub mod lower;
                    #[path = "../../../../../../../../✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🔨️modules/📝️writer/🦀️.rs"]
                    pub mod writer;
                }
            }
        }
    }
}
