/** @emoji 🎯 S-play shooting VCS handler with mesh input binding. */

import { createTypedAppVcsHandler } from "@semio-tech/s-core";
import type { PlatformDefinition } from "@semio-tech/framework-platform-core";
import { DEFAULT_SHOOTING_FIXTURE, type ShootingFixture } from "@semio-tech/shooting-react";

import { shootingPlayAppDefinition } from "@semio-tech/shooting-core";

export function buildShootingProgramDefinition(): PlatformDefinition {
	const app = shootingPlayAppDefinition;
	return {
		id: "shooting",
		name: "Shooting",
		apiVersion: "1",
		apps: [{ id: "shooting", label: app.label, controllerId: app.controllerId, modes: app.modes, defaultModeId: app.defaultModeId }],
		createPlatformApi: () => ({}),
	};
}

export function createSPlayShootingAppVcsHandler() {
	return createTypedAppVcsHandler<ShootingFixture, { readonly op: "noop" }>(
		"shooting.scene",
		"shooting.fixture",
		() => DEFAULT_SHOOTING_FIXTURE,
		(fixture) => fixture,
		undefined,
		{
			applyInputBindings: (fixture, inputBindings) => {
				const mesh = inputBindings.mesh as { readonly url?: string } | undefined;
				if (!mesh?.url) return fixture;
				const activeId = fixture.activeAssetId ?? fixture.assets[0]?.id;
				if (!activeId) return fixture;
				return {
					...fixture,
					assets: fixture.assets.map((asset) => (asset.id === activeId ? { ...asset, url: mesh.url! } : asset)),
				};
			},
		},
	);
}
