#!/usr/bin/env python3
"""Turns the official memo test into a standalone dev test (tmpdir fixtures, no harness): devtest.py <in> <out>."""
import sys
s = open(sys.argv[1], encoding="utf8").read()
def once(a, b):
    global s
    assert s.count(a) == 1, a[:80]
    s = s.replace(a, b)
once('test("the source memo answers exactly what a full capture answers after every kind of edit",()=>withBuildTestInvocationV1(async original=>withRepositoryTestArtifactsV1(process.cwd(),original,createIssuedResourceControlV1(original,"test:source-memo-artifacts",0),async()=>{',
     'import{tmpdir}from"node:os";\ntest("the source memo answers exactly what a full capture answers after every kind of edit",async()=>{')
once('const artifacts=repositoryTestArtifactRootV1(process.cwd()),', 'const artifacts=tmpdir(),')
once('})),120000);', '},120000);')
for line in ['import{withBuildTestInvocationV1}from', 'import{createIssuedResourceControlV1}from', 'import{withRepositoryTestArtifactsV1,repositoryTestArtifactRootV1}from']:
    i = s.index(line); j = s.index("\n", i); s = s[:i] + s[j + 1:]
open(sys.argv[2], "w", encoding="utf8").write(s)
