import sys
p = sys.argv[1]
s = open(p, encoding='utf-8', newline='').read()
cmd_anchor = '''                #[path = "."]
                pub mod place_elements {'''
assert s.count(cmd_anchor) == 1
cmd = '''                #[path = "."]
                pub mod select_findings {
                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🎯️select-findings/🦀️.rs"]
                    mod component;
                    pub use component::*;
                }
'''
panel_anchor = '''                #[path = "."]
                pub mod library {
                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/🛍️library/🦀️.rs"]
                    mod component;
                    pub use component::*;
                }
'''
assert s.count(panel_anchor) == 1
panel = '''                #[path = "."]
                pub mod diagnostics {
                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/🚨️diagnostics/🦀️.rs"]
                    mod component;
                    pub use component::*;
                }
'''
s = s.replace(cmd_anchor, cmd + cmd_anchor).replace(panel_anchor, panel_anchor + panel)
open(p, 'w', encoding='utf-8', newline='').write(s)
