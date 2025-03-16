import type { ForgeConfig } from '@electron-forge/shared-types';
// import { MakerDeb } from '@electron-forge/maker-deb';
// import { MakerSquirrel } from '@electron-forge/maker-squirrel';
// import { MakerZIP } from '@electron-forge/maker-zip';


const config: ForgeConfig = {
    plugins: [
        {
            name: '@electron-forge/plugin-vite',
            config: {
                // Main process, Preload scripts, Worker process, etc.
                build: [
                    {
                        entry: 'main.ts',
                        // config: '../core/vite.config.mts'
                        // config: 'vite.main.config.mts'
                        config: 'vite.main.config.ts'
                    },
                    {
                        entry: 'preload.ts',
                        // config: '../core/vite.config.mts'
                        config: 'vite.preload.config.ts'
                    }
                ],
                renderer: [
                    {
                        name: 'main_window',
                        // config: '../core/vite.config.mts'
                        config: 'vite.renderer.config.ts'
                    }
                ]
            }
        }
    ],
    // packagerConfig: {
    //     osxSign: {},
    //     osxNotarize: {
    //         tool: 'notarytool',
    //         appleId: process.env.APPLE_ID,
    //         appleIdPassword: process.env.APPLE_PASSWORD,
    //         teamId: process.env.APPLE_TEAM_ID
    //     }
    // },
    // makers: [
    // new MakerSquirrel({
    //     authors: 'Electron contributors'
    // }, ['win32']),
    // new MakerZIP({}, ['darwin']),
    // new MakerDeb({}, ['linux']),
    // new MakerRpm({}, ['linux']),
    // ],
    publishers: [
        {
            name: '@electron-forge/publisher-github',
            config: {
                repository: {
                    owner: 'usalu',
                    name: 'semio'
                },
                prerelease: false,
                draft: true
            }
        }
    ]
}

export default config;