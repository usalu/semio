from pathlib import Path
base=Path('✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🧬️schema/⚖️validation')
for filename in ['evaluator.rs','population.rs']:
 p=base/filename;s=p.read_text(encoding='utf-8').replace('self.control.maximum_depth','self.control.maximum_depth.min(256)');p.write_text(s,encoding='utf-8',newline='\n')
p=base/'native-execution.test.ts';s=p.read_text(encoding='utf-8').replace('flow and independent expected values','flow and shared expected values');p.write_text(s,encoding='utf-8',newline='\n')