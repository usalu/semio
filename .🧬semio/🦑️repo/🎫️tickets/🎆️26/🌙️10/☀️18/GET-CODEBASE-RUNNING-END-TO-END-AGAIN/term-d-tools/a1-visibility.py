import os
os.chdir('/Users/ueli/Documents/semio')
D='🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🖥️terminal'
def edit(rel, pairs):
    p=D+'/'+rel
    s=open(p,encoding='utf-8').read()
    for old,new,count in pairs:
        assert s.count(old)==count,(rel,old,s.count(old))
        s=s.replace(old,new)
    open(p,'w',encoding='utf-8').write(s)
edit('🪟️windows/🦀️.rs',[
 ('pub(super) enum Body {','pub enum Body {',1),
 ('pub(super) enum Kind {','pub enum Kind {',1),
 ("pub(super) enum Place<'a>","pub enum Place<'a>",1),
 ('pub(super) struct DashboardWindow {','pub struct DashboardWindow {',1),
 ('pub(super) enum LinkState {','pub enum LinkState {',1),
 ('pub(super) struct ContextMenu {','pub struct ContextMenu {',1),
 ('pub(super) enum Effect {','pub enum Effect {',1),
 ('pub(super) struct Dashboard {','pub struct Dashboard {',1),
 ('pub(super) fn node_constraint()','pub fn node_constraint()',1),
 ('pub(super) fn stack_of(','pub fn stack_of(',1),
])
edit('📋️panes/🦀️.rs',[
 ('pub(super) enum RowAction','pub enum RowAction',1),
 ('pub(super) struct Pane {','pub struct Pane {',1),
 ('pub(super) struct PreferenceJob','pub struct PreferenceJob',1),
])
edit('📡️sessions/🦀️.rs',[
 ('pub(super) struct Link {','pub struct Link {',1),
 ('pub(super) struct Polled','pub struct Polled',1),
 ('    #[cfg(test)]\n    pub online_for_test: bool,\n    #[cfg(test)]\n    pub fail_sends: bool,','    #[cfg(feature = "testkit")]\n    pub online_for_test: bool,\n    #[cfg(feature = "testkit")]\n    pub fail_sends: bool,',1),
 ('#[cfg(test)] online_for_test: false, #[cfg(test)] fail_sends: false','#[cfg(feature = "testkit")] online_for_test: false, #[cfg(feature = "testkit")] fail_sends: false',1),
 ('fn pretend_online(&self) -> bool { #[cfg(test)] { self.online_for_test } #[cfg(not(test))] { false } }','fn pretend_online(&self) -> bool { #[cfg(feature = "testkit")] { self.online_for_test } #[cfg(not(feature = "testkit"))] { false } }',1),
 ('fn fails(&self) -> bool { #[cfg(test)] { self.fail_sends } #[cfg(not(test))] { false } }','fn fails(&self) -> bool { #[cfg(feature = "testkit")] { self.fail_sends } #[cfg(not(feature = "testkit"))] { false } }',1),
 ('    #[cfg(test)]\n    pub fn next_attempt','    #[cfg(feature = "testkit")]\n    pub fn next_attempt',1),
 ('    #[cfg(test)]\n    pub fn backoff','    #[cfg(feature = "testkit")]\n    pub fn backoff',1),
 ('    #[cfg(test)]\n    pub fn offline','    #[cfg(feature = "testkit")]\n    pub fn offline',1),
])
edit('⌨️controls/🦀️.rs',[
 ('pub(super) enum Action{','pub enum Action{',1),
 ('pub(super) enum Flow','pub enum Flow',1),
 ('pub(super) fn fit_hints(','pub fn fit_hints(',1),
 ('pub(super) fn spec_of(','pub fn spec_of(',1),
 ('pub(super) fn event_of(','pub fn event_of(',1),
])
