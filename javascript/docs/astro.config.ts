// @ts-check
import { defineConfig } from 'astro/config';
import starlight from '@astrojs/starlight';
import react from '@astrojs/react';
import mdx from '@astrojs/mdx';
// import tailwind from '@astrojs/tailwind';
// import tailwindcss from '@tailwindcss/vite';

// https://astro.build/config
export default defineConfig({
    integrations: [starlight({
        title: 'semio Docs',
        social: {
            github: 'https://github.com/usalu/semio',
        },
        sidebar: [
            {
                label: 'Guides',
                items: [
                    // Each item here is one entry in the navigation menu.
                    { label: 'Example Guide', slug: 'guides/example' },
                ],
            },
            {
                label: 'Reference',
                autogenerate: { directory: 'reference' },
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