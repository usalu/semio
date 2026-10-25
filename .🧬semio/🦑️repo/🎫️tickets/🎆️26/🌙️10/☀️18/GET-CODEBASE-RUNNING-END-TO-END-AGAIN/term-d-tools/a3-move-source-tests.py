import os,re
os.chdir('/Users/ueli/Documents/semio')
D='🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🖥️terminal'
p=D+'/📂️sources/🦀️.rs'
s=open(p,encoding='utf-8').read()
marker='\n#[cfg(test)]\nmod tests {\n'
assert s.count(marker)==1
head,tests=s.split(marker)
assert tests.rstrip().endswith('}')
body=tests.rstrip()[:-1].rstrip('\n')+'\n'
head=head.rstrip('\n')+'\n'
body=re.sub(r'^[ \t]*println!\("\[DEBUG\][^\n]*\n','',body,flags=re.M)
body=body.replace('fixture_text!("../../📂️sources/🧪️tests/🔣️.json")','fixture_text!("../../../../../📂️sources/🧪️tests/🔣️.json")') if False else body
header='''/// 📂️ Source windows: browsing, saving, conflicts, guided file commands and worker settlement through the real view.

use dashboard_terminal::testing::{Body, Dashboard, Kind, Place};
use dashboard_source_view::{EntryAction, Request};
use semio_framework_filesystem_workspace::commands::{self, DocumentBuffer, Message, Response};
use semio_framework_filesystem_workspace::{Progress, Stage, Workspace};
use semio_framework_ui_tui_core::event::{Event, Key, KeyEvent};
use semio_framework_ui_tui_engine::Tui;
use semio_framework_ui_tui_widgets::widget::{EditorState, WidgetState};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{mpsc, Arc};
'''
body=body.replace('    use super::*;\n','',1)
lines=[l[4:] if l.startswith('    ') else l for l in body.split('\n')]
out=header+'\n'+'\n'.join(lines)
os.makedirs(D+'/🧪️tests/🔬️unit/📂️sources',exist_ok=True)
open(D+'/🧪️tests/🔬️unit/📂️sources/🦀️.rs','w',encoding='utf-8').write(out)
open(p,'w',encoding='utf-8').write(head)
print(len(out),len(head))
