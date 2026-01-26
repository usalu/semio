#!/bin/bash
go/repo/repo graphql '{"query": "{ folder(path: \".\") { id path children { path } files { path } } }"}'
