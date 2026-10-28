from pathlib import Path
p=Path('✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🔎️details')
def write(f,s):
 b=s.encode('utf8');old=f.stat().st_size
 with f.open('r+b')as o:o.write(b);o.write(b' '*max(0,old-len(b)))
f=p/'🦀️.rs';s=f.read_text(encoding='utf8');a='    fn direct_attribute_count(name:&str)->Option<usize>{Some(Self::catalog().entities.get(name)?.attributes.len())}'
s=s.replace(a,a+'\n    /// 🌱️ A rootless declaration proves canonical full and direct tuple positions coincide.\n    fn rootless_attribute_count(name:&str)->Option<usize>{let declaration=Self::catalog().entities.get(name)?;declaration.supertype.is_none().then_some(declaration.all_attributes.len())}')
write(f,s)
f=Path('✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/✏️editor/details.rs');s=f.read_text(encoding='utf8');a='    fn direct_attribute_count(name:&str)->Option<usize>{Some(crate::validation::metadata::entity(name)?.attributes.len())}';s=s.replace(a,a+'\n    fn rootless_attribute_count(name:&str)->Option<usize>{let declaration=crate::validation::metadata::entity(name)?;declaration.supertype.is_none().then_some(declaration.all_attributes.len())}');write(f,s)
f=p/'constituent_roster.rs';s=f.read_text(encoding='utf8');s=s.replace('    mapping: bool,','    mapping: bool,\n    repair: bool,\n    rootless_profile: Option<usize>,')
a='    pub(super) fn primary_arity(&self) -> Option<usize> {'
s=s.replace(a,'''    pub(super) fn primary_name<'a>(&self, entity: &'a IfcEntity, mutations: &'a [IfcMutation]) -> Option<&'a str> {
        if !self.owner.state.primary { return None; }
        if self.owner.state.repair { return Some(&entity.name); }
        match mutations.first()? { IfcMutation::SetEntityName(value) if value.id == entity.id => Some(&value.name), _ => None }
    }
'''+a)
a='''            (true, Some(IfcMutation::SetEntityName(candidate))) => {'''
s=s.replace(a,'''            (true, _) if state.repair => {
                state.mapping && state.arity == state.target_arity && entity.args.len() == state.arity
                    && self.owner.primitive_count() == Some(mutations.len())
                    && self.owner.root_slots.as_ref().is_some_and(|slots| slots.len() == state.target_arity)
                    && ConstituentRoster::type_name(event,address).is_some_and(|name|name.eq_ignore_ascii_case(&entity.name))
                    && (!entity.complex.is_empty() || state.rootless_profile == Some(C::rootless_attribute_count as usize) && C::rootless_attribute_count(&entity.name) == Some(state.arity))
                    && matches!(mutations.first(), Some(IfcMutation::SetEntityArg(value)) if value.id == state.id)
            }
'''+a)
a='''                    let mapping = complete && (!self.state.primary || !entity.complex.is_empty()) && old != next && old != Some(0) && next != Some(0);
                    let supplied = super::type_input::rows(event).is_some();'''
b='''                    let supplied = super::type_input::rows(event).is_some();
                    let repair = complete && self.state.primary && supplied && old == next && old != Some(0)
                        && Self::type_name(event,address).is_some_and(|requested|requested.eq_ignore_ascii_case(name))
                        && (!entity.complex.is_empty() || super::domain_cursor::canonical_name(name,&mut previous).and_then(C::rootless_attribute_count) == old);
                    let mapping = repair || complete && (!self.state.primary || !entity.complex.is_empty()) && old != next && old != Some(0) && next != Some(0);'''
assert a in s;s=s.replace(a,b).replace('IFC type inputs require complete complex-primary differing-arity remap','IFC type inputs require an original complete primary remap')
s=s.replace('                        self.state.mapping = mapping;','                        self.state.mapping = mapping;\n                        self.state.repair = repair;\n                        self.state.rootless_profile = (repair && entity.complex.is_empty()).then_some(C::rootless_attribute_count as usize);')
a='''                let name = Self::type_name_mut(event, address).ok_or_else(|| invalid("IFC primary original type name owner missing"))?;
                name.make_ascii_uppercase();
                let values = pending.get_or_insert_with(Vec::new);
                if *name != snapshot.entities[self.state.position].name {
                    values.try_reserve_exact(count).map_err(|_| ValueError::literal(ValueRefusalKind::AllocationFailed, "IFC primary type primitive allocation failed"))?;
                    cx.consume_retained(RetainedCloneProgress { retained_capacity_bytes: values.capacity() * size_of::<IfcMutation>(), ..Default::default() })?;
                    values.push(IfcMutation::SetEntityName(semio_s_artifact_stdio_ifc_4_mutations::standards::v4::subsets::any::schema::mutations::set_entity_name::SetEntityName { id: self.state.id, name: std::mem::take(name) }));
                }'''
b='''                if self.state.repair && count == 0 { return Err(invalid("IFC unchanged type has no required argument changes").into()); }
                let name = Self::type_name_mut(event, address).ok_or_else(|| invalid("IFC primary original type name owner missing"))?;
                let values = pending.get_or_insert_with(Vec::new);
                if self.state.repair || !name.eq_ignore_ascii_case(&snapshot.entities[self.state.position].name) {
                    values.try_reserve_exact(count).map_err(|_| ValueError::literal(ValueRefusalKind::AllocationFailed, "IFC primary type primitive allocation failed"))?;
                    cx.consume_retained(RetainedCloneProgress { retained_capacity_bytes: values.capacity() * size_of::<IfcMutation>(), ..Default::default() })?;
                    if !self.state.repair {
                        name.make_ascii_uppercase();
                        values.push(IfcMutation::SetEntityName(semio_s_artifact_stdio_ifc_4_mutations::standards::v4::subsets::any::schema::mutations::set_entity_name::SetEntityName { id: self.state.id, name: std::mem::take(name) }));
                    }
                }'''
assert a in s;s=s.replace(a,b)
s=s.replace('self.state.arity.saturating_sub(self.state.target_arity).checked_add(1)', 'self.state.arity.saturating_sub(self.state.target_arity).checked_add(usize::from(!self.state.repair))')
a='''                if values.len() <= removals {
                    values.push(IfcMutation::RemoveEntityArg(RemoveEntityArg { id: self.state.id, index: self.state.arity - values.len() }));'''
b='''                let removed = values.len().saturating_sub(usize::from(!self.state.repair));
                if removed < removals {
                    values.push(IfcMutation::RemoveEntityArg(RemoveEntityArg { id: self.state.id, index: self.state.arity - removed - 1 }));'''
assert a in s;s=s.replace(a,b);write(f,s)
f=p/'domain_work.rs';s=f.read_text(encoding='utf8')
a='''        let Some(mutation) = mutations.get(self.mutation) else {'''
s=s.replace(a,'''        if let Some(admission) = strict_candidate.filter(|admission|admission.primary_arity().is_some()) {
            let name = admission.primary_name(entity,mutations).ok_or("ifc-domain-primary-name")?;
            let mut bytes=[0;256];
            let declaration=C::catalog().entities.get(canonical_name(name,&mut bytes).ok_or("ifc-domain-entity")?).ok_or("ifc-domain-entity")?;
            let count=admission.primary_arity().ok_or("ifc-domain-primary-role")?;
            if count!=declaration.attributes.len(){return Err("ifc-domain-argument-arity".into());}
            if self.argument==count {self.done=true;return Ok(None);}
            return Ok(Some(Task{before:None,after:admission.primary_argument(entity,mutations,self.argument).ok_or("ifc-domain-primary-argument")?,attribute:Self::attribute::<C>(name,self.argument,true)?,index:self.argument,part:None,creation:false}));
        }
'''+a,1)
a=s.index('            IfcMutation::SetEntityName(value) if strict_candidate.is_some() => {');b=s.index('            IfcMutation::SetEntityComplex(value) => {',a);s=s[:a]+s[b:]
a='''        let primary = retained_type.as_ref().and_then(|admission| match mutations.first() {
            Some(IfcMutation::SetEntityName(value)) => Some((admission.position(), value.name.as_str())),
            _ => None,
        });'''
b='''        let primary = retained_type.as_ref().and_then(|admission|admission.primary_name(entity,mutations).map(|name|(admission.position(),name)));'''
assert a in s;s=s.replace(a,b);write(f,s)
