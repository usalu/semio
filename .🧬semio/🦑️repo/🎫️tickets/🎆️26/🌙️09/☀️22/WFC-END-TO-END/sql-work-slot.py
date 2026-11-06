from pathlib import Path

p=Path('🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🏛️model/🦀️.rs')
s=p.read_text()
s=s.replace("work:Option<&'a mut dyn SqliteSnapshotWorkRecipient>, required_allocation", "work:Option<&'a mut dyn SqliteSnapshotWorkRecipient>, work_failure:Option<&'a mut Option<SqliteStageWorkFailure>>, required_allocation",1)
s=s.replace('Self { work:None, required_allocation', 'Self { work:None, work_failure:None, required_allocation',1)
s=s.replace("pub fn install_work_recipient(&mut self,work:&'a mut dyn SqliteSnapshotWorkRecipient)->std::result::Result<(),ValueError>{", "pub fn install_work_recipient(&mut self,work:&'a mut dyn SqliteSnapshotWorkRecipient,failure:&'a mut Option<SqliteStageWorkFailure>)->std::result::Result<(),ValueError>{")
s=s.replace('if self.work.is_some(){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"SQLite cannot replace its original work recipient"))}\n        self.ledger.work_baseline=self.ledger.performed_work;self.work=Some(work);Ok(())', 'if self.work.is_some()||self.work_failure.is_some()||failure.is_some(){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"SQLite requires a vacant original work recipient and failure slot"))}\n        self.ledger.work_baseline=self.ledger.performed_work;self.work=Some(work);self.work_failure=Some(failure);Ok(())')
old='work:self.work.as_mut().map(|work|&mut **work as &mut dyn SqliteSnapshotWorkRecipient),required_allocation:'
assert s.count(old)==3
s=s.replace(old,'work:self.work.as_mut().map(|work|&mut **work as &mut dyn SqliteSnapshotWorkRecipient),work_failure:self.work_failure.as_deref_mut(),required_allocation:')
s=s.replace('        let mut work_failure=None;\n        let(turn,declared)={','        if self.work_failure.as_deref().is_some_and(Option::is_some){return semio_framework_value::RetainedWorkOutcome{result:Err(SqliteStageFailure::NotStarted(ValueError::literal(ValueRefusalKind::InvariantViolated,"SQLite original work failure is unsettled"))),work:0}}\n        let(turn,declared)={')
s=s.replace('baseline:*work_baseline,failure:&mut work_failure','baseline:*work_baseline,failure:self.work_failure.as_deref_mut()')
s=s.replace('        let result=match(work_failure,allocation){','        let work_failure=self.work_failure.as_deref_mut().and_then(Option::take);\n        let result=match(work_failure,allocation){')
p.write_text(s)
