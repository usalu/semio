# storage test edits: the remote part of the Bazel cache flags now comes from the neutral contract.

edit(STORAGE_TEST, r''',remote=cachePolicy.remote,
   rows=database.query("SELECT printf('build --experimental_disk_cache_gc_idle_delay=%ds',?1) AS line UNION ALL SELECT printf('build --experimental_disk_cache_gc_max_age=%dd',?2) UNION ALL SELECT printf('build --repo_contents_cache_gc_max_age=%dd',?3) UNION ALL SELECT 'build --remote_cache_compression' WHERE ?4 UNION ALL SELECT printf('build --remote_timeout=%ds',?5) UNION ALL SELECT printf('build --remote_retries=%d',?6)").all(idle,age,repositoryAge,remote.compression?1:0,remote.timeoutSeconds,remote.retries) as {line:string}[];''',
     r''',
   rows=database.query("SELECT printf('build --experimental_disk_cache_gc_idle_delay=%ds',?1) AS line UNION ALL SELECT printf('build --experimental_disk_cache_gc_max_age=%dd',?2) UNION ALL SELECT printf('build --repo_contents_cache_gc_max_age=%dd',?3)").all(idle,age,repositoryAge) as {line:string}[];''')

edit(STORAGE_TEST,
     'expect(rows.map(row=>row.line)).toEqual(cache.bazelrc);for(const line of cache.bazelrc)expect(lines).toContain(line);',
     'expect(rows.map(row=>row.line)).toEqual(cache.bazelrc);for(const line of cache.bazelrc)expect(lines).toContain(line);expect(text,"remote cache flags come only from the neutral contract").not.toMatch(/remote_/u);')

edit(STORAGE_TEST,
     ' for(const name of [cachePolicy.remote.endpointEnvironment,cachePolicy.remote.uploadEnvironment,cachePolicy.remote.credentialHelperEnvironment])expect(Object.keys(componentSchema.definitions.CacheEnvironment.properties)).toContain(name);\n',
     '')

region(STORAGE_TEST, '   const helper=join(directory,"helper.sh")', 'spawnSync(binary,["--batch"', r'''   const flags=bazelCacheFlagsV1(layout,workspace,{environment:row.environment,skipCache:(row as {skipCache?:boolean}).skipCache});if(flags.helper){if(process.platform==="win32")continue;mkdirSync(layout.temporary,{recursive:true});writeFileSync(flags.helper.path,flags.helper.content,{mode:flags.helper.mode});}
   const result=''')
