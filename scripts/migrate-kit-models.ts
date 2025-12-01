/**
 * Migration script: kit_metabolism.json.old -> kit_metabolism.json
 *
 * Migrates `representations` from old kit format to `models` in new kit format.
 *
 * Schema changes:
 * - OLD: `representations` array on types
 * - NEW: `models` array on types
 *
 * Type identification changes:
 * - OLD: `name` + `variant` (variant can be empty)
 * - NEW: `name` (variant becomes child type name with `parent.guid`)
 *
 * Model/representation structure remains similar:
 * - `url`, `description`, `tags`, `attributes` stay the same
 * - NEW: `guid` added to each model
 */

import { randomUUID } from 'crypto';
import { readFileSync, writeFileSync } from 'fs';
import { resolve } from 'path';

interface OldRepresentation {
    url: string;
    description: string;
    tags: string[];
    attributes: unknown[];
}

interface OldType {
    name: string;
    variant: string;
    representations: OldRepresentation[];
    // ... other fields
}

interface OldKit {
    name: string;
    description: string;
    icon: string;
    image: string;
    preview: string;
    version: string;
    remote: string;
    homepage: string;
    license: string;
    types: OldType[];
    // ... other fields
}

interface NewModel {
    guid: string;
    url: string;
    description?: string;
    tags?: string[];
    attributes?: unknown[];
}

interface NewType {
    guid: string;
    name: string;
    parent?: { guid: string };
    models?: NewModel[];
    // ... other fields
}

interface NewKit {
    guid: string;
    name?: string;
    description?: string;
    icon?: string;
    image?: string;
    preview?: string;
    version?: string;
    remote?: string;
    homepage?: string;
    license?: string;
    types: NewType[];
    // ... other fields
}

function main() {
    const basePath = resolve(__dirname, '../assets/semio');
    const oldKitPath = resolve(basePath, 'kit_metabolism.json.old');
    const newKitPath = resolve(basePath, 'kit_metabolism.json');

    console.log('Reading old kit...');
    const oldKit: OldKit = JSON.parse(readFileSync(oldKitPath, 'utf-8'));

    console.log('Reading new kit...');
    const newKit: NewKit = JSON.parse(readFileSync(newKitPath, 'utf-8'));

    // Migrate kit-level metadata
    console.log('\nMigrating kit metadata...');
    if (oldKit.name && !newKit.name) {
        newKit.name = oldKit.name;
        console.log(`  [OK] name: ${oldKit.name}`);
    }
    if (oldKit.description && !newKit.description) {
        newKit.description = oldKit.description;
        console.log(`  [OK] description: ${oldKit.description.substring(0, 50)}...`);
    }
    if (oldKit.icon && !newKit.icon) {
        newKit.icon = oldKit.icon;
        console.log(`  [OK] icon: ${oldKit.icon}`);
    }
    if (oldKit.image && !newKit.image) {
        newKit.image = oldKit.image;
        console.log(`  [OK] image: ${oldKit.image}`);
    }
    if (oldKit.preview && !newKit.preview) {
        newKit.preview = oldKit.preview;
        console.log(`  [OK] preview: ${oldKit.preview}`);
    }
    if (oldKit.remote && !newKit.remote) {
        newKit.remote = oldKit.remote;
        console.log(`  [OK] remote: ${oldKit.remote}`);
    }
    if (oldKit.homepage && !newKit.homepage) {
        newKit.homepage = oldKit.homepage;
        console.log(`  [OK] homepage: ${oldKit.homepage}`);
    }
    if (oldKit.license && !newKit.license) {
        newKit.license = oldKit.license;
        console.log(`  [OK] license: ${oldKit.license}`);
    }

    // Build a map of new types by guid for parent lookup
    const newTypesByGuid = new Map<string, NewType>();
    for (const type of newKit.types) {
        newTypesByGuid.set(type.guid, type);
    }

    // Mapping from old type names to new parent names
    // The new kit simplified some naming conventions
    const parentNameMapping: Record<string, string> = {
        'Capsule with Balcony': 'Balcony',
        'Ellipsoid Capsule': 'Ellipsoid',
        'Trapezoid Capsule': 'Trapezoid',
    };

    // Build mapping from old (name, variant) to new type
    // Logic:
    // - If old variant is empty -> find new type with same name and no parent
    // - If old variant is not empty -> find new type where:
    //   - name matches variant
    //   - parent.guid refers to a type whose name matches mapped oldName
    function findNewType(oldName: string, oldVariant: string): NewType | undefined {
        if (!oldVariant) {
            // Root type: find by name, no parent
            return newKit.types.find((t) => t.name === oldName && !t.parent);
        } else {
            // Variant type: name is the variant, parent's name is (mapped) oldName
            const mappedParentName = parentNameMapping[oldName] || oldName;
            return newKit.types.find((t) => {
                if (t.name !== oldVariant) return false;
                if (!t.parent) return false;
                const parentType = newTypesByGuid.get(t.parent.guid);
                return parentType && parentType.name === mappedParentName;
            });
        }
    }

    console.log(`\nMigrating ${oldKit.types.length} types...`);

    let migratedCount = 0;
    let skippedCount = 0;
    const notFound: string[] = [];

    for (const oldType of oldKit.types) {
        const oldId = oldType.variant ? `${oldType.name}:${oldType.variant}` : oldType.name;

        if (!oldType.representations || oldType.representations.length === 0) {
            console.log(`  [SKIP] ${oldId} - no representations`);
            skippedCount++;
            continue;
        }

        const newType = findNewType(oldType.name, oldType.variant);

        if (!newType) {
            console.log(`  [NOT FOUND] ${oldId}`);
            notFound.push(oldId);
            continue;
        }

        // Convert representations to models
        const models: NewModel[] = oldType.representations.map((rep) => ({
            guid: randomUUID(),
            url: rep.url,
            ...(rep.description && { description: rep.description }),
            ...(rep.tags && rep.tags.length > 0 && { tags: rep.tags }),
            ...(rep.attributes && rep.attributes.length > 0 && { attributes: rep.attributes }),
        }));

        newType.models = models;
        console.log(`  [OK] ${oldId} -> ${newType.name} (${models.length} models)`);
        migratedCount++;
    }

    console.log(`\n--- Summary ---`);
    console.log(`Migrated: ${migratedCount}`);
    console.log(`Skipped (no representations): ${skippedCount}`);
    console.log(`Not found: ${notFound.length}`);
    if (notFound.length > 0) {
        console.log(`  Not found types: ${notFound.join(', ')}`);
    }

    // Write updated kit
    console.log(`\nWriting updated kit to ${newKitPath}...`);
    writeFileSync(newKitPath, JSON.stringify(newKit, null, 2));
    console.log('Done!');
}

main();
