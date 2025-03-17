
import React from 'react'
import ReactDOM from 'react-dom/client'

import './globals.css'

// import Diagram from '@semio/core'

function App() {
    return (
        <h1 className='text-pink-600 text-center'>semio </h1>
    );
}

export default App;

ReactDOM.createRoot(document.getElementById('root')!).render(
    <React.StrictMode>
        <App />
    </React.StrictMode>,
)