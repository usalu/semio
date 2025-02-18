// @ts-check
import { defineConfig } from 'astro/config';
import starlight from '@astrojs/starlight';
import react from '@astrojs/react';
import mdx from '@astrojs/mdx';
// import tailwind from '@astrojs/tailwind';
// import tailwindcss from '@tailwindcss/vite';

// https://astro.build/config
export default defineConfig({
    site: 'https://docs.semio-tech.com',
    integrations: [starlight({
        title: {
            en: 'Docs',
            'de': 'Dokumentation',
        },
        defaultLocale: 'root',
        locales: {
            root: {
                label: 'English',
                lang: 'en',
            },
            'de': {
                label: 'Deutsch',
                lang: 'de',
            },
        },
        social: {
            github: 'https://github.com/usalu/semio',
            discord: 'https://discord.gg/m6nnf6pQRc'
        },
        logo: {
            light: './src/assets/emblem_round.svg',
            dark: './src/assets/emblem_dark_round.svg',
        },
        editLink: {
            baseUrl: 'https://github.com/usalu/semio/edit/main/javascript/docs',
        },
        sidebar: [
            {
                label: 'Tutorials',
                autogenerate: { directory: 'tutorials' }
            },
            {
                label: 'Installation',
                items: [
                    { label: 'Getting Started', slug: 'installation' }
                ]
            },
        ],
        customCss: ['./globals.css'],
    }), react(), mdx(),
        // tailwind({
        // applyBaseStyles: false, // Disable default base styles
        // }),
    ],
    // vite: {
    // plugins: [tailwindcss()],
    // },
});