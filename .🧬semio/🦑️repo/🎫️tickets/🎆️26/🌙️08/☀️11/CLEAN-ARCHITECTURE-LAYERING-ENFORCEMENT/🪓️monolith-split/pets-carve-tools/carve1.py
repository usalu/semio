import os,re,sys
from plan import *

def read(p): return open(p,encoding='utf-8').read()
def write(p,s):
    os.makedirs(os.path.dirname(p),exist_ok=True)
    open(p,'w',encoding='utf-8').write(s)

KIT_DIR=PETS+'/🧬️schema/🧪️tests/🧰️testkit'
KIT=KIT_DIR+'/🦀️.rs'
SCHEMA_TEST=PETS+'/🧬️schema/🧪️tests/🔬️unit/🦀️.rs'

def step_testkit():
    s=read(SCHEMA_TEST)
    a=s.index('pub(crate) fn fixture(case: &str) -> Value {')
    b=s.index('fn round_trips<T: Serialize + DeserializeOwned>')
    assert s.count('pub(crate) fn fixture(case: &str) -> Value {')==1 and s.count('fn round_trips<T: Serialize + DeserializeOwned>')==1
    kit=s[a:b].rstrip()+'\n'
    kit=kit.replace('pub(crate) fn','pub fn')
    assert '"../../🧫️fixtures"' in kit
    kit=kit.replace('"../../🧫️fixtures"','"../../../../🧪️tests/🧫️fixtures"')
    head='''//! 🧰️ The test kit the unit suites of every pets crate share: the shared fixture documents read from `🧫️fixtures`, typed decoding and encoding through serde, the number and bit helpers, and the exact structural comparison. Compiled for the unit tests of the contract crate and for the `testkit` feature that the unit tests of its dependants switch on.
//!
//! @see ../../🦀️.rs — the schema twin the kit decodes with
//! @see ../../../🧪️tests/🧫️fixtures — the committed vectors

use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_json::Value;
use std::path::PathBuf;

'''
    write(KIT,head+kit)
    rest=s[:a]+s[b:]
    used=[n for n in ('assert_same','bits','entries','fixture','json','number','typed') if re.search(r'\b'+n+r'\b',rest.replace('use super::*;',''))]
    return rest,used

def finish_schema_test(rest,used):
    names=[]
    for n in ('assert_same','bits','entries','fixture','json','number','typed'):
        body=rest
        pat={'json':r'(?<![!\w])json\(','fixture':r'\bfixture\(','typed':r'\btyped(::<|\()','entries':r'\bentries\(','number':r'\bnumber\(','bits':r'\bbits\(','assert_same':r'\bassert_same\('}[n]
        if re.search(pat,body): names.append(n)
    old='use super::*;\nuse serde::de::DeserializeOwned;\nuse serde_json::{json, Value};\nuse std::path::PathBuf;\n'
    assert rest.count(old)==1
    new='use super::*;\nuse crate::testkit::{'+', '.join(names)+'};\nuse serde::de::DeserializeOwned;\nuse serde_json::{json, Value};\n'
    rest=rest.replace(old,new)
    write(SCHEMA_TEST,rest)
    return names

if __name__=='__main__':
    if sys.argv[1]=='testkit':
        rest,used=step_testkit()
        names=finish_schema_test(rest,used)
        print('schema test imports',names)
