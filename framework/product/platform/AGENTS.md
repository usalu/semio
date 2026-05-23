# Components

## Table

## VirtualFileSystem

Hierarchical file-system table (`virtualFileSystem` component kind). Each {@link AppRuntime} binds its own surface via {@link virtualFileSystemSurfaceId} and {@link registerAppVirtualFileSystem}. Controllers extend {@link VirtualFileSystemController}; state is keyed by `appId` + `surfaceId`, and child nodes load only for expanded branches.

## Puzzle2d

## Puzzle3d

## Puzzle5d

