---
technology: semio
bundle:
 name: js
 emoji: 📜
 description: The js bundle for semio.
 kind: library
---

# 🧾 Specification

## 🕸️ Systems

## 🧮 Algorithms

## 🛠️ Mechanisms

## 📛 Entities

### Kit

```ts
interface Kit {
    uuid: string
    name: string
    …
    types: Type[]
    designs: Design[]
}

const KitSnapshotSchema = z.object(…)
type KitSnapshot = z.infer<typeof KitSchema> // DTO

interface KitSelection {
    types: TypeId[]
    designs: DesignId[]
}

interface KitInteraction {
    uuid: string
    selection: KitSelection
}

const KitInteractionSchema = z.object(…)
type KitInteractionDTO = z.infer<typeof KitInteractionSchema> // DTO

class KitInteractionEntity implements KitInteraction {
    …
}

const KitInteractionEntitySchema = z.object(…)
type KitInteractionEntityDTO = z.infer<typeof KitInteractionEntitySchema> // DTO

interface SynchronizedKit extends Kit {
    import(kit: Kit): Promise<SynchronizedKit>
    export(): Promise<Kit>
    open(options: any): Promise<SynchronizedKit>
    close(): Promise<void>
    interactions:
}

class KitEntity implements Kit {
    …
}

class LocalKitEntity extends KitEntity {
    …
}

class DevKitEntity extends KitEntity {
    …
}

class RemoteKitEntity extends KitEntity {
    …
}

const k:SynchronizedKit = new FolderKitEntity("some/folder/path")

const i1:string = k.interactions.start("Some ui interaction") // uuid-v7
const i2:string = k.interactions.start("Some concurrent ai interaction")
k.setActiveInteraction(i1)
// regular OO way of working - interactions dont leak into any CRUD
const nakaginCapsuleTowerDesign:Design = k.findDesign(name="Nakagin Capsule Tower")
nakaginCapsuleTowerDesign.flatten() // adds a kit change to interaction stack
nakaginCapsuleTowerDesign.findPiece(name="c1").delete() // adds another kit change
k.undo() // undo last active interaction e.g. deletion of c1 piece
k.setActiveInteraction(i2) // changes active interaction but leaves
const anotherPiece = nakaginCapsuleTowerDesign.findPiece(name="c2") // the instance doesnt change because
anotherPiece.changeType(c1TA[1]) //
k.unsetActiveInteraction()

k.undo() // undo last history change by applying the last backward

```
