//! 🧪️ The shared selection corpus retains all current present-owner obligations.
use super::*;
#[test]
fn shared_owner_selection(){
 let cases=&[
  ("specific-artifact-missing-recipe","✏️s/plugin/artifact",Some("artifact"),false,true),
  ("specific-artifact-declared","✏️s/plugin/artifact",Some("artifact"),true,true),
  ("stdio-missing-recipe","✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/format",Some("artifact"),false,true),
  ("wfc-missing-recipe","✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/◻️2d",Some("artifact"),false,true),
  ("specific-plugin","✏️s/plugin",Some("plugin"),true,false),
  ("specific-test-leaf","✏️s/test",Some("test"),true,false),
  ("specific-no-role","✏️s/artifact",None,true,false),
  ("generic-contributed-artifact","🧰️framework/artifact",Some("artifact"),true,true),
  ("generic-uncontributed-artifact","🧰️framework/artifact",Some("artifact"),false,false),
  ("explicit-other-area-artifact","🌎️hub/artifact",Some("artifact"),true,true),
  ("other-area-uncontributed-artifact","🌎️hub/artifact",Some("artifact"),false,false),
  ("specific-prefix-lookalike","✏️sibling/artifact",Some("artifact"),false,false),
  ("specific-product","✏️s/product",Some("product"),true,false),
  ("repo-root",".",Some("artifact"),false,false),
 ];
 for(id,directory,role,declaration_present,eligible)in cases{assert_eq!(current_fixture_contribution_eligible(directory,*role,*declaration_present),*eligible,"{id}");}
}
