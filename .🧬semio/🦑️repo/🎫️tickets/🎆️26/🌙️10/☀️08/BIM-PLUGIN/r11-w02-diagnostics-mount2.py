import sys
p = sys.argv[1]
s = open(p, encoding='utf-8', newline='').read()
anchor = '''                #[path = "."]
                pub mod place_elements {'''
assert s.count(anchor) == 1
block = ''
for name, dirname in (("set_classification", "🗂️set-classification"), ("remove_classification", "🗄️remove-classification")):
    block += '''                #[path = "."]
                pub mod %s {
                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/%s/🦀️.rs"]
                    mod component;
                    pub use component::*;
                }
''' % (name, dirname)
s = s.replace(anchor, block + anchor)
open(p, 'w', encoding='utf-8', newline='').write(s)
