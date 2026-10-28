from pathlib import Path
p=Path('✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🔎️details/type-guidance.rs')
s=p.read_text(encoding='utf-8').replace('68 => {\n                demand.capacity_bytes = self.choice_path_len()? * size_of::<String>();','68 => {\n                demand.capacity_bytes = 4 * size_of::<String>();').replace('path.try_reserve_exact(path_count).map_err(|_| ValueError::literal(ValueRefusalKind::AllocationFailed, "type page address"))','path.try_reserve_exact(4).map_err(|_| ValueError::literal(ValueRefusalKind::AllocationFailed, "type page address"))')
p.write_text(s,encoding='utf-8',newline='\n')
