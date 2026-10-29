module=importlib.reload(module)
result=unittest.TestResult();start79=time.monotonic()
unittest.defaultTestLoader.loadTestsFromTestCase(module.SlabPartitionContract).run(result)
record={'source':hashlib.sha256(Path('/repo/🎓️teaching/🏛️architecture/🏗️bim/🎬️clip/🧱️model/🔣️.json').read_bytes()).hexdigest(),'run':result.testsRun,'seconds':time.monotonic()-start79,'failures':[(str(t),m) for t,m in result.failures],'errors':[(str(t),m) for t,m in result.errors]}
Path('/repo/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️27/CONCRETE-BIM-CLIP-EXAMPLES/🗑️generated/slab-full-r79.json').write_text(json.dumps(record,ensure_ascii=False),encoding='utf8')
print({'run':result.testsRun,'seconds':record['seconds'],'failures':len(result.failures),'errors':len(result.errors)})
